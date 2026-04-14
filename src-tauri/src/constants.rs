pub const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug)]
pub enum GlobalEvents{
    SettingsUpdated
}
impl GlobalEvents{
    pub const fn as_str(&self) -> &str{
        match *self{
            Self::SettingsUpdated => "settings-updated",
        }
    }
}
