mod models;
mod parser;
mod utils;
mod csv_writer;

use crate::models::Invoice;
use tauri::State;
use std::sync::Mutex;
use std::path::Path;
use std::fs;
use std::ffi::OsStr;

struct AppState {
    invoices: Mutex<Vec<Invoice>>,
}

#[tauri::command]
fn scan_folder(folder_path: String) -> Result<Vec<Invoice>, String> {
    let path = Path::new(&folder_path);
    if !path.exists() || !path.is_dir() {
        return Err("Ungültiges Verzeichnis".to_string());
    }

    let doc_xml = path.join("document.xml");
    let mut invoices = vec![];

    if doc_xml.exists() {
        let doc_info = parser::parse_document_xml(&doc_xml);
        for (xml_file, pdf_file) in doc_info {
            let xml_path = path.join(&xml_file);
            
            if xml_path.exists() {
                if let Some(mut inv) = parser::parse_invoice_xml(&xml_path) {
                    // Start with provided PDF or empty
                    let mut final_pdf = pdf_file.clone();
                    
                    // FALLBACK 1: If no PDF from document.xml, try same name as XML
                    if final_pdf.is_empty() {
                        let pdf_guess = xml_path.with_extension("pdf");
                        if pdf_guess.exists() {
                            final_pdf = pdf_guess.file_name().unwrap_or_default().to_string_lossy().to_string();
                        }
                    }

                    // Store absolute local path
                    if !final_pdf.is_empty() {
                        inv.pdf_filename = Some(path.join(final_pdf).to_string_lossy().to_string());
                    } else {
                        // FALLBACK 2: Just look for any PDF that has the ID in name if possible
                        // But for now, we leave it empty if not found
                        inv.pdf_filename = None;
                    }

                    invoices.push(inv);
                }
            }
        }
    } else {
        // Fallback: search all XML files if no document.xml
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let p = entry.path();
                if p.extension().map_or(false, |ext| ext == "xml") && p.file_name() != Some(OsStr::new("document.xml")) {
                    if let Some(mut inv) = parser::parse_invoice_xml(&p) {
                        let pdf_guess = p.with_extension("pdf");
                        if pdf_guess.exists() {
                            inv.pdf_filename = Some(pdf_guess.to_string_lossy().to_string());
                        }
                        invoices.push(inv);
                    }
                }
            }
        }
    }

    Ok(invoices)
}

#[tauri::command]
fn generate_csv(invoices: Vec<Invoice>, target_file: String, start_nr: i32, symbol: String) -> Result<String, String> {
    let path = Path::new(&target_file);
    csv_writer::write_bmd_csv(&invoices, path, start_nr, &symbol)?;
    
    // Save last beleg nr
    let selected_count = invoices.iter().filter(|i| i.selected).count() as i32;
    let next_nr = start_nr + selected_count - 1;
    let _ = std::fs::write("../data/letzte_belegnr.txt", (next_nr).to_string());
    
    Ok("CSV erfolgreich erstellt!".to_string())
}

#[tauri::command]
fn get_last_beleg_nr() -> i32 {
    let path = Path::new("../data/letzte_belegnr.txt");
    if let Ok(content) = std::fs::read_to_string(path) {
        if let Ok(nr) = content.trim().parse::<i32>() {
            return nr;
        }
    }
    0
}

#[tauri::command]
fn get_base_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { invoices: Mutex::new(vec![]) })
        .invoke_handler(tauri::generate_handler![
            scan_folder, 
            generate_csv, 
            get_last_beleg_nr,
            get_base_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
