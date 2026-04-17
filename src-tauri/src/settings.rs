use std::{
    fmt::Display,
    fs::{create_dir, create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{constants::SETTINGS_FILE_NAME, error::AppError};

use std::sync::Mutex;

use log::{error, info};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::constants::GlobalEvents;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub is_darkmode: bool,
    pub project_folder: Option<PathBuf>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            is_darkmode: true,
            project_folder: None,
        }
    }
}

impl Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "is_darkmode: {}", self.is_darkmode)
    }
}

impl AppSettings {
    pub fn set_project_folder(&mut self, project_folder_path: PathBuf) {
        self.project_folder = Some(project_folder_path);
    }
    pub fn save_to_file(&self, app_config_dir: PathBuf) -> Result<(), AppError> {
        let err: AppError = AppError::Unknown("AppSettings save_to_file".to_string());
        create_dir_all(app_config_dir.clone()).map_err(|_| err.clone())?;
        let app_settings_json_str = serde_json::to_string(self).map_err(|_| err.clone())?;
        let mut app_settings_file_path = app_config_dir;
        app_settings_file_path.push(SETTINGS_FILE_NAME);
        write(app_settings_file_path, app_settings_json_str).map_err(|_| err.clone())?;
        Ok(())
    }
    pub fn read_from_file(app_config_dir: PathBuf) -> Result<Self, AppError> {
        let err: AppError = AppError::Unknown("AppSettings read_from_file".to_string());
        let mut app_settings_file_path = app_config_dir;
        app_settings_file_path.push(SETTINGS_FILE_NAME);
        let app_settings_json_str =
            read_to_string(app_settings_file_path).map_err(|_| err.clone())?;
        let app_settings: AppSettings =
            serde_json::from_str(&app_settings_json_str).map_err(|_| err.clone())?;
        Ok(app_settings)
    }
}

// --- Tauri Commands ---

#[tauri::command]
pub fn test_change_settings(app: AppHandle, state: State<'_, Mutex<AppSettings>>) {
    let mut state = state.lock().expect("Should be able to aquire lock");
    (*state).is_darkmode = true;

    app.emit(GlobalEvents::SettingsUpdated.as_str(), state.clone())
        .unwrap();
}

#[tauri::command]
pub fn get_settings(state: State<'_, Mutex<AppSettings>>) -> AppSettings {
    let state = state.lock().expect("Should be able to aquire lock");
    state.clone()
}

#[tauri::command]
pub async fn pick_project_folder(
    app: AppHandle,
    state: State<'_, Mutex<AppSettings>>,
) -> Result<(), String> {
    let mut state = state.lock().expect("Should be able to aquire lock");
    let folder_path = app.dialog().file().blocking_pick_folder();
    let folder_path = match folder_path {
        Some(fp) => fp,
        None => {
            return Ok(());
        }
    }
    .into_path();
    let project_folder_path = match folder_path {
        Ok(fp) => fp,
        Err(e) => {
            error!("{}", e);
            return Err("Unable to get PathBuf".to_string());
        }
    };
    state.set_project_folder(project_folder_path);
    if let Err(e) = state.save_to_file(app.path().app_config_dir().expect("Should be fine")) {
        error!("{}", e);
    };
    app.emit(GlobalEvents::SettingsUpdated.as_str(), state.clone())
        .unwrap();
    Ok(())
}
