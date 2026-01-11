//! Core library for the Time Grasp application.
//!
//! This crate exposes the core application layers used by the Tauri
//! frontend: context helpers, model types, persistence/store adapters, and
//! a small backend-model-controller (BMC) layer used by IPC handlers.
//!
//! The module layout is intentionally small and focused so application code
//! can import only the pieces it needs.
#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::sync::Arc;

use backend::error;
use backend::ipc;
use backend::store;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> error::Result<()> {
    println!("building app");
    tauri::Builder::default()
        .setup(|app| {
            // let win = app.get_webview_window("main").unwrap();
            // #[cfg(debug_assertions)]
            // {
            //     // Helps a lot for "why is it still old"
            //     win.open_devtools();
            //     // Some platforms expose webview APIs differently; devtools + hard reload is the win.
            // }
            println!("creating store");
            let store_manager = store::StoreManager::from_path(app.path().app_data_dir()?);
            println!("created store");
            app.manage(Arc::new(store_manager));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Category Group
            ipc::create_group,
            ipc::read_group,
            ipc::list_group,
            ipc::update_group,
            ipc::delete_group,
            // Category
            ipc::create_category,
            ipc::read_category,
            ipc::list_category,
            ipc::update_category,
            ipc::delete_category,
            // Entry
            ipc::create_entry,
            ipc::read_entry,
            ipc::list_entry,
            ipc::update_entry,
            ipc::delete_entry,
        ])
        .run(tauri::generate_context!())?;
    println!("built app");

    Ok(())
}
