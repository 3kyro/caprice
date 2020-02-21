use std::fmt::Display;
use std::sync::mpsc;

pub type Result<T> = std::result::Result<T, CapriceError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum CapriceError {
    CrosstermError(crossterm::ErrorKind),
    Io(std::io::Error),
    SendErr(mpsc::SendError<String>),
}

impl std::error::Error for CapriceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CapriceError::CrosstermError(e) => Some(e),
            CapriceError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for CapriceError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CapriceError::CrosstermError(e) => write!(fmt, "Terminal error occured: {}", e),
            CapriceError::Io(e) => write!(fmt, "IO-error occurred: {}", e),
            CapriceError::SendErr(e) => write!(fmt, "Send error occured: {}", e),
        }
    }
}

#[macro_export]
macro_rules! impl_from {
    ($from:path, $to:expr) => {
        impl From<$from> for CapriceError {
            fn from(e: $from) -> Self {
                $to(e)
            }
        }
    };
}

impl_from!(crossterm::ErrorKind, CapriceError::CrosstermError);
impl_from!(std::io::Error, CapriceError::Io);
impl_from!(mpsc::SendError<String>, CapriceError::SendErr);
