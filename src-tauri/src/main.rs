#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use printpdf::*;
use std::convert::From;


#[tauri::command]
fn gen_pdf_report(title: &str, report: &str) -> Vec<u8> {
  let (doc, page1, layer1) = PdfDocument::new(title, Mm(210.0), Mm(297.0), "Report");

  let current_layer = doc.get_page(page1).get_layer(layer1);
  let mut font_reader = std::io::Cursor::new(include_bytes!("../assets/fonts/arial.ttf").as_ref());
  let font = doc.add_external_font(&mut font_reader).unwrap();
  current_layer.use_text(report, 12.0, Mm(10.0), Mm(287.0), &font);
  
  doc.save_to_bytes().unwrap()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![gen_pdf_report])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
