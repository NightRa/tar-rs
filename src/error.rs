use std::error;
use std::fmt;
use std::io::{self, Error};

#[derive(Debug)]
pub struct TarError {
    desc: String,
    io: io::Error,
}

impl TarError {
    pub fn new(desc: &str, err: Error) -> TarError {
        TarError {
            desc: desc.to_string(),
            io: err,
        }
    }
}

impl error::Error for TarError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(&self.io)
    }
}

impl fmt::Display for TarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.desc, self.io)
    }
}

impl From<TarError> for Error {
    fn from(t: TarError) -> Error {
        Error::new(t.io.kind(), t)
    }
}
