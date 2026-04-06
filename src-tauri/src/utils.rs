use rust_decimal::Decimal;
use std::path::Path;

pub fn format_eu_number(val: Decimal) -> String {
    format!("{:.2}", val).replace(".", ",")
}

pub fn map_to_ts_client_path(local_path: &str) -> String {
    if local_path.is_empty() { return String::new(); }
    
    // Convert C:\Path\To\File.pdf to \\tsclient\C\Path\To\File.pdf
    let path = local_path.replace("/", "\\");
    if let Some(pos) = path.find(":") {
        let drive = &path[..pos];
        let rest = &path[pos + 1..];
        return format!(r"\\tsclient\{}{}", drive, rest);
    }
    path
}

pub fn get_next_beleg_num(last: i32) -> String {
    format!("{:05}", last + 1)
}
