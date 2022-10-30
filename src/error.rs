use core::fmt;
use std::{
    fmt::{Debug, Display},
    io,
    str::Utf8Error,
};

pub struct HttpError {
    pub kind: String,
    pub message: String,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpError{{kind:{},message:{}}}",
            self.kind, self.message
        )
    }
}

impl Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpError")
            .field("error", &self.kind)
            .field("message", &self.message)
            .finish()
    }
}

impl From<io::Error> for HttpError {
    fn from(error: io::Error) -> Self {
        HttpError {
            kind: "io".to_string(),
            message: error.to_string(),
        }
    }
}

impl From<Utf8Error> for HttpError {
    fn from(error: Utf8Error) -> Self {
        HttpError {
            kind: "utf8".to_string(),
            message: error.to_string(),
        }
    }
}

pub struct ThreadError {
    pub kind: String,
    pub message: String,
}

impl Display for ThreadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ThreadError{{kind:{},message:{}}}",
            self.kind, self.message
        )
    }
}

impl Debug for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadError")
            .field("error", &self.kind)
            .field("message", &self.message)
            .finish()
    }
}
