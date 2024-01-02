pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidLevel,
    InvalidLifetime,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidLevel => "invalid level".to_string(),
            Error::InvalidLifetime => "invalid lifetime".to_string(),
        }
    }
}
