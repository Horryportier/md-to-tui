use std::fmt::Display;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    ParserErr(),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
