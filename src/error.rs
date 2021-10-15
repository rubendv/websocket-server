use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: Option<String>,
    source: Box<dyn std::error::Error>
}

impl Error {
    pub fn new<E: Into<Box<dyn std::error::Error>>>(message: &str, e: E) -> Self {
        Error {
            message: Some(message.into()),
            source: e.into()
        }
    }
    fn from<E: Into<Box<dyn std::error::Error>>>(e: E) -> Self {
        Error {
            message: None,
            source: e.into()
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::from(e)
    }
}

impl std::convert::From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self {
        Self::from(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref message) = self.message {
            write!(f, "Error: {}. Caused by: {}", message, self.source)
        } else {
            self.source.fmt(f)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}