use std::str::Utf8Error;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    QuickXml(quick_xml::Error),
    Decode(Utf8Error),
    Unknown,
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Error::QuickXml(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Decode(e)
    }
}
