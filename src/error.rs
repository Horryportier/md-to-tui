use std::{fmt::{Display, format}, default};

#[derive(Debug, thiserror::Error, PartialEq, Clone, Default)]
pub enum Error {
    #[default]
    DefErr, 

    ParserErr(String),
    LexerErr(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String;  
        match self {
            Self::DefErr =>  text = "Deafult error".into(),
            Self::ParserErr(s) => text = format!("Parser Err {}", s),
            Self::LexerErr(s) => text = format!("Parser Err {}", s),
        }
        write!(f, "md-to-tui error:  {}", text)
    }
}
