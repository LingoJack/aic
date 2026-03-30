use std::fmt;

#[derive(Debug)]
pub enum AicError {
    UnknownKey(String),
    UnknownModifier(String),
    EventCreationFailed(String),
    ScreenshotFailed(String),
    ImageEncodingFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for AicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AicError::UnknownKey(k) => write!(f, "Unknown key: '{k}'"),
            AicError::UnknownModifier(m) => write!(f, "Unknown modifier: '{m}'"),
            AicError::EventCreationFailed(msg) => write!(f, "Failed to create event: {msg}"),
            AicError::ScreenshotFailed(msg) => write!(f, "Screenshot failed: {msg}"),
            AicError::ImageEncodingFailed(msg) => write!(f, "Image encoding failed: {msg}"),
            AicError::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for AicError {}

impl From<std::io::Error> for AicError {
    fn from(e: std::io::Error) -> Self {
        AicError::IoError(e)
    }
}
