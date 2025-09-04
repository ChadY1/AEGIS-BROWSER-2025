#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod net_intercept;
mod cookies;
mod history;
mod crypto;
mod prefs;
mod firewall_proxy;

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      // init here if needed
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("app failed");
}
