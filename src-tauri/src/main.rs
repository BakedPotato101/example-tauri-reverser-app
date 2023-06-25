// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub fn reverse(string: &str) -> String {
  let str_vec: Vec<char> = string.chars().rev().collect();
  return str_vec.into_iter().collect(); 
}



#[tauri::command]
fn greet(name: &str) -> String {
  let revname = &reverse(name);
  format!("Your word reversed is {}", revname)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
