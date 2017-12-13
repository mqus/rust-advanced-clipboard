use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoDataError{}

impl Error for NoDataError{
    fn description(&self) -> &str {
        "no content in this clipboard"
    }
}

impl fmt::Display for NoDataError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
