mod api;
mod core;
mod pipeline;
mod services;
mod sources;

use api::commands::{attach_pipeline, list_parsers, list_sources, mock_rx, start_source, stop_source};
use services::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            list_sources,
            list_parsers,
            start_source,
            stop_source,
            attach_pipeline,
            mock_rx
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
