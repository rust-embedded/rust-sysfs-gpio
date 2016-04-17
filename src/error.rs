use std::convert;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    /// Simple IO error
    Io(io::Error),
    /// Read unusual data from sysfs file.
    Unexpected(String),
    /// Invalid Path
    InvalidPath(String),
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Unexpected(_) => "An Unexpected Error Occurred",
            Error::InvalidPath(_) => "A Provided Path was invalid",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::Unexpected(ref s) => write!(f, "Unexpected: {}", s),
            Error::InvalidPath(ref s) => write!(f, "Invalid Path: {}", s),
        }
    }
}


impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl convert::From<::nix::Error> for Error {
    fn from(e: ::nix::Error) -> Error {
        Error::Io(io::Error::from_raw_os_error(e.errno() as i32))
    }
}
