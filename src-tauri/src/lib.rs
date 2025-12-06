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

// pub mod bmc;
// pub mod ctx;
// pub mod error;
// pub mod ipc;
// pub mod store;

use backend::error;
use backend::ipc;
use backend::store;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_and_read_back_group(
    app: tauri::AppHandle<tauri::Wry>,
    name: &str,
) -> error::Result<String> {
    let data = model::CategoryGroupForCreate {
        id: model::Uuid::new_v4(),
        name: name.to_string(),
        note: None,
    };
    let id = ipc::create_group(app.clone(), data).await?;
    let group = ipc::read_group(app, id).await?;
    Ok(format!("Category Group created: {:?}", group))
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> error::Result<()> {
    println!("loading store");
    let store_manager = store::StoreManager::new().await?;
    println!("loaded store");
    let store_manager = Arc::new(store_manager);

    println!("building app");
    tauri::Builder::default()
        .manage(store_manager)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Greet
            greet,
            create_and_read_back_group,
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
            // Category
            ipc::create_entry,
            ipc::read_entry,
            ipc::list_entry,
            ipc::update_entry,
            ipc::delete_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("built app");

    Ok(())
}
