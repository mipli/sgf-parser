use std::error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidProperty(String),
    Unkown
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match *self {
            InvalidProperty(ref prop) => write!(f, "{}: ({})", self.description(), prop),
            Unkown => write!(f, "{}", self.description())
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            InvalidProperty(_) => "Invalid property value",
            Unkown => "Uknown error",
        }
    }
}
