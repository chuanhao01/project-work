use std::sync::Mutex;

use log::{error, info};
use tauri::{AppHandle, Emitter, Manager, State};

use crate::{constants::GlobalEvents, settings::AppSettings};

mod constants;
mod error;
mod settings;

#[tauri::command]
fn greet(state: State<'_, Mutex<AppSettings>>, name: &str) -> String {
    let state = state.lock().expect("Should be able to aquire lock");
    info!("{}", state);
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            settings::get_settings,
            settings::test_change_settings,
            settings::pick_project_folder
        ])
        .setup(|app| {
            let app_handle = app.app_handle();
            let app_settings =
                AppSettings::read_from_file(app_handle.path().app_config_dir().expect("App Setup"))
                    .unwrap_or_else(|_| AppSettings::default());

            // println!(
            //     "{}",
            //     app_handle
            //         .path()
            //         .app_config_dir()
            //         .unwrap()
            //         .to_str()
            //         .unwrap()
            // );
            // println!("{}", app_settings);
            app.manage(Mutex::new(app_settings.clone()));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
