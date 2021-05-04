use std::fmt;
#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub explanation: Option<String>,
}

#[derive(Debug)]
pub enum Kind {
    IOError(std::io::Error),
    UnknownError,
}

impl Error {
    pub fn new(kind: Kind) -> Self {
        Error {
            kind,
            explanation: None,
        }
    }
    pub fn explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_owned());
        self
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            Kind::IOError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref e) = self.explanation {
            write!(f, "{}", e)?;
        }
        match &self.kind {
            Kind::IOError(e) => write!(f, "IO Error, caused by: {}", e),
            Kind::UnknownError => write!(f, "Unknown Error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            kind: Kind::IOError(e),
            explanation: None,
        }
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Error {
            kind: Kind::UnknownError,
            explanation: None,
        }
    }
}
