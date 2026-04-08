use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceEntry {
    pub bp_account_no: Option<String>,
    pub account_no: String,
    pub amount_net: Decimal,
    pub amount_gross: Decimal,
    pub tax_percent: Decimal,
    pub tax_amount: Decimal,
    pub date: NaiveDate,
    pub supplier_name: String,
    pub information: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
    pub consolidated_invoice_id: String,
    pub consolidated_amount: Decimal,
    pub pdf_filename: Option<String>,
    pub entries: Vec<InvoiceEntry>,
    pub selected: bool, // For UI selection
    pub status: String,
}
