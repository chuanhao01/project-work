use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AppError {
    Unknown(String),
}

impl Display for AppError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Unknown(e) => write!(f, "{}", e)
        }
    }
}
