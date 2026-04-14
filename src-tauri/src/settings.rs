use std::{
    fmt::Display,
    fs::{create_dir, create_dir_all, read_to_string, write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{constants::SETTINGS_FILE_NAME, error::AppError};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub is_darkmode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self { is_darkmode: true }
    }
}

impl Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "is_darkmode: {}", self.is_darkmode)
    }
}

impl AppSettings {
    pub fn save_to_file(app_settings: Self, app_config_dir: PathBuf) -> Result<(), AppError> {
        let err: AppError = AppError::Unknown("AppSettings save_to_file".to_string());
        create_dir_all(app_config_dir.clone()).map_err(|_|err.clone())?;
        let app_settings_json_str =
            serde_json::to_string(&app_settings).map_err(|_| err.clone())?;
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
