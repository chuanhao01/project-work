use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct AppSettings{
    is_darkmode: bool,
}

impl Default for AppSettings{
    fn default() -> Self {
        Self { is_darkmode: true }
    }
}

impl AppSettings{
    pub fn save_to_file(app_settings: Self, ) -> Result<(), AppError>{
        Ok(())
    }
}
