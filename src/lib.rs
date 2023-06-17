#![allow(unused_imports)]
#![warn(missing_docs)]
//!
use error::Error;
use ratatui::text::Text;
mod error;
mod parser;
mod style;


/// trait MarkdownParsable will take AsRef<[u8]> and parse it into ratatui Text
pub trait MarkdownParsable {
    /// Convert type to Text
    fn parse_markdown(&self) -> Result<Text<'static>, Error>;
}

impl<T> MarkdownParsable for T where T: AsRef<[u8]> {
    fn parse_markdown(&self) -> Result<Text<'static>, Error> {
        Err(Error::ParserErr())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
