use crate::models::Invoice;
use crate::utils::{format_eu_number, map_to_ts_client_path};
use std::fs::File;
use std::io::Write;
use encoding_rs::WINDOWS_1252;
use std::path::Path;

pub fn write_bmd_csv(invoices: &[Invoice], target_path: &Path, start_beleg_nr: i32, symbol: &str) -> Result<(), String> {
    let mut current_nr = start_beleg_nr;
    let mut output = String::new();
    
    // Header
    let headers = [
        "satzart", "konto", "gkonto", "buchdatum", "belegdatum", "belegnr", 
        "betrag", "text", "buchsymbol", "buchcode", "periode", "uva-periode", 
        "uva-kursdatum", "steuercode", "prozent", "steuer", "uidnr", 
        "dokument", "extbelegnr"
    ];
    output.push_str(&headers.join(";"));
    output.push('\n');
    
    for inv in invoices {
        if !inv.selected { continue; }
        
        let belegnr_str = current_nr.to_string();
        current_nr += 1;
        
        let doc_path = if let Some(pdf) = &inv.pdf_filename {
            map_to_ts_client_path(pdf)
        } else {
            String::new()
        };
        
        for entry in &inv.entries {
            let date_str = entry.date.format("%d.%m.%Y").to_string();
            let period_str = entry.date.format("%m").to_string()
                                .parse::<i32>().unwrap().to_string();
            let uva_period_str = entry.date.format("%Y%m").to_string();
            
            let fields = [
                "0".to_string(), // satzart
                entry.bp_account_no.as_deref().unwrap_or("99990").to_string(), // konto
                entry.account_no.clone(), // gkonto
                date_str.clone(), // buchdatum
                date_str.clone(), // belegdatum
                belegnr_str.clone(), // belegnr
                format_eu_number(entry.amount_gross), // betrag
                format!("{} {} {}", symbol, entry.supplier_name, inv.consolidated_invoice_id), // Use dynamic symbol
                symbol.to_string(), // buchsymbol (Dynamic)
                "1".to_string(), // buchcode
                period_str.clone(), // periode
                uva_period_str.clone(), // uva-periode
                date_str.clone(), // uva-kursdatum
                "".to_string(), // steuercode
                format_eu_number(entry.tax_percent), // prozent
                "".to_string(), // steuer
                "".to_string(), // uidnr
                doc_path.clone(), // dokument
                inv.consolidated_invoice_id.clone() // extbelegnr
            ];
            output.push_str(&fields.join(";"));
            output.push('\n');
        }
    }
    
    // Convert to Windows-1252 (ANSI)
    let (encoded, _, _) = WINDOWS_1252.encode(&output);
    let mut file = File::create(target_path).map_err(|e| e.to_string())?;
    file.write_all(&encoded).map_err(|e| e.to_string())?;
    
    Ok(())
}
