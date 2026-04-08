use crate::models::{Invoice, InvoiceEntry};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::path::Path;
use std::fs;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use std::str::FromStr;

fn clean_cdata(s: String) -> String {
    s.replace("<![CDATA[", "").replace("]]>", "")
}

pub fn parse_document_xml(filepath: &Path) -> Vec<(String, String)> {
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    
    let mut results = vec![];
    let mut current_xml = String::new();
    let mut current_pdf = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"document" => {
                current_xml = String::new();
                current_pdf = String::new();
            }
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"extension" => {
                let mut data_file = String::new();
                let mut file_name = String::new();

                for attr in e.attributes() {
                    if let Ok(a) = attr {
                        let key = String::from_utf8_lossy(a.key.as_ref()).to_lowercase();
                        let val = String::from_utf8_lossy(&a.value).into_owned();
                        
                        // Just look for the defining attributes directly
                        if key == "datafile" {
                            data_file = val;
                        } else if key == "name" {
                            file_name = val;
                        }
                    }
                }

                if !data_file.is_empty() {
                    current_xml = data_file;
                }
                if file_name.to_lowercase().ends_with(".pdf") && !file_name.to_lowercase().contains("summary") {
                    current_pdf = file_name;
                }
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"document" => {
                if !current_xml.is_empty() {
                    results.push((current_xml.clone(), current_pdf.clone()));
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => (),
        }
        buf.clear();
    }
    results
}

pub fn parse_invoice_xml(filepath: &Path) -> Option<Invoice> {
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut inv_id = String::new();
    let mut inv_amount = Decimal::ZERO;
    let mut inv_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    
    let mut entries = vec![];

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"consolidate" => {
                for attr in e.attributes() {
                    if let Ok(a) = attr {
                        let key = String::from_utf8_lossy(a.key.as_ref()).into_owned();
                        let val = String::from_utf8_lossy(&a.value).into_owned();
                        match key.as_str() {
                            "consolidatedInvoiceId" => inv_id = val,
                            "consolidatedAmount" => inv_amount = Decimal::from_str(&val.replace(",", ".")).unwrap_or(Decimal::ZERO),
                            "consolidatedDate" => {
                                inv_date = NaiveDate::parse_from_str(&val, "%Y-%m-%d").unwrap_or(inv_date);
                            },
                            _ => ()
                        }
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"accountsPayableLedger" => {
                let mut e_date = inv_date;
                let mut e_amount = Decimal::ZERO;
                let mut e_acc = String::new();
                let mut e_bp_acc = None;
                let mut e_tax = Decimal::ZERO;
                let mut e_supplier = String::new();
                let mut e_info = None;

                let mut ledger_buf = Vec::new();
                loop {
                    match reader.read_event_into(&mut ledger_buf) {
                        Ok(Event::Start(ref le)) => {
                            let tag = String::from_utf8_lossy(le.local_name().as_ref()).to_lowercase();
                            let text = match reader.read_text(le.name()) {
                                Ok(t) => clean_cdata(t.into_owned()),
                                Err(_) => String::new(),
                            };
                            match tag.as_str() {
                                "date" => e_date = NaiveDate::parse_from_str(&text, "%Y-%m-%d").unwrap_or(e_date),
                                "amount" => e_amount = Decimal::from_str(&text.replace(",", ".")).unwrap_or(Decimal::ZERO),
                                "accountno" => e_acc = text,
                                "bpaccountno" => e_bp_acc = Some(text),
                                "tax" => e_tax = Decimal::from_str(&text.replace(",", ".")).unwrap_or(Decimal::ZERO),
                                "suppliername" => e_supplier = text,
                                "information" => if !text.is_empty() { e_info = Some(text) },
                                _ => ()
                            }
                        }
                        Ok(Event::End(ref le)) if le.local_name().as_ref() == b"accountsPayableLedger" => break,
                        Ok(Event::Eof) => break,
                        _ => ()
                    }
                    ledger_buf.clear();
                }

                entries.push(InvoiceEntry {
                    bp_account_no: e_bp_acc,
                    account_no: e_acc,
                    amount_net: e_amount - e_tax,
                    amount_gross: e_amount,
                    tax_percent: e_tax,
                    tax_amount: Decimal::ZERO,
                    date: e_date,
                    supplier_name: e_supplier,
                    information: e_info,
                });
            }
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    if inv_id.is_empty() && entries.is_empty() { return None; }

    Some(Invoice {
        consolidated_invoice_id: inv_id,
        consolidated_amount: inv_amount,
        pdf_filename: None,
        entries,
        selected: true,
        status: "Gelesen".to_string(),
    })
}
