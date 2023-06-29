use ratatui::{text::Span, style::Style};

use crate::style::style::MdStyle;

use super::lexer::Token;


pub fn generate_heading(heading: &usize, style: &MdStyle) -> Span<'static>{
    Span::styled("#".repeat(heading.clone()), style.heading)
}

pub fn generate_indent(indent: String, style: &MdStyle) -> Span<'static> {
        Span::styled(indent, style.text)
}

pub fn genarate_list_start(token: Token,  style: &MdStyle) -> Span<'static> {
    match token {
        Token::Plus => Span::styled("+", style.list),
        Token::Dash => Span::styled("-", style.list),
        Token::Asterisk => Span::styled("*", style.list),
        Token::Indent(i) => Span::styled(format!("{}", i) ,style.list),
        _ => Span::from("")
    }
}

pub fn generate_horizontal_rule(token: Token, style: &MdStyle) -> Span<'static> {
    match token {
        Token::Equal => Span::styled("===", style.horizontal_rule),
        Token::Undersocre => Span::styled("___", style.horizontal_rule),
        Token::Dash => Span::styled("---", style.horizontal_rule),
        _ => Span::from(""),
    }
}
