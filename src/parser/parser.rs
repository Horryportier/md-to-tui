use std::usize;

use ratatui::text::{Line, Span};

use crate::{error::Error, style::style::MdStyle};

use super::{
    lexer::Token,
    parser_helpers::{
        genarate_list_start, generate_heading, generate_horizontal_rule, generate_indent,
    },
};

// #[derive(Debug, PartialEq, Clone, Default)]
// enum Expr {
//     Heading(Vec<Token>),
//     List(Vec<Token>),
//     Blockquete(Vec<Token>),
//     Paragrath(Vec<Token>),
//     LinkText(Vec<Token>),
//     Link(Vec<Token>),
//     Tag(Vec<Token>),
//     Empty,
//     #[default]
//     None,
// }

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Parser {
    pub input: Vec<Token>,
    pub style: MdStyle,

    pub position: usize,
    pub read_position: usize,
    pub token: Token,

    pub lines: Vec<Line<'static>>,
}

impl Parser {
    pub fn new(input: Vec<Token>, style: Option<MdStyle>) -> Parser {
        // info!("created new Parser");

        let style = match style {
            Some(style) => style,
            None => MdStyle::default(),
        };

        let parser = Parser {
            input,
            style,
            ..Default::default()
        };

        // info!("Parser {:?}", parser);

        return parser;
    }

    pub fn parse(&mut self) -> Result<Vec<Line<'static>>, Error> {
        self.read_token();

        let mut lines: Vec<Line> = Vec::new();
        self.next_line()?;
        let mut end = !(self.position >= self.input.len());
        while end {
            // info!("IS_END {}", end);
            // info!("Positon/Len/Tk {}/{}/{}", self.position, self.input.len(), self.token);
            let next = self.next_line()?;
            // info!("Next Line {:#?}", next);
            end = !(self.position >= self.input.len());
            lines.push(next);
        }

        // info!("Lines {:#?}", lines);

        self.lines = lines.clone();
        Ok(lines)
    }

    fn next_line<'a>(&mut self) -> Result<Line<'a>, Error> {
        let mut spans: Vec<Span> = Vec::new();
        while !self.token.is_end() {
            // info!("Is end {}", self.token.is_end());

            spans.push(match &self.token {
                Token::Heading(heading) => generate_heading(heading, &self.style),
                Token::Asterisk | Token::Dash | Token::Plus => {
                    genarate_list_start(self.token.clone(), &self.style)
                }
                Token::WhiteSpace => Span::from(" "),
                Token::Indent(i) => generate_indent(i.into(), &self.style),
                Token::Equal => {
                    if self.peek() == Token::Equal {
                        if self.peek() == Token::Equal {
                            generate_horizontal_rule(self.token.clone(), &self.style);
                        };
                        generate_indent("==".into(), &self.style);
                    }
                    generate_indent("=".into(), &self.style)
                }
                Token::Undersocre => {
                    if self.peek() == Token::Undersocre {
                        if self.peek() == Token::Undersocre {
                            generate_horizontal_rule(self.token.clone(), &self.style);
                        };
                        generate_indent("__".into(), &self.style);
                    }
                    generate_indent("_".into(), &self.style)
                }
                Token::Dot => Span::from("."),
                Token::RightParen => Span::styled("(", self.style.link),
                Token::LeftParen => Span::styled(")", self.style.link),
                Token::LeftSquare=> Span::styled("[", self.style.link_text),
                Token::RightSquare => Span::styled("]", self.style.link_text),
                Token::RightAngle => Span::styled(">", self.style.blocqoutes),
                Token::LeftAngle => Span::styled("<", self.style.blocqoutes),
                Token::BackTick => Span::styled("`", self.style.backtick),
                Token::Colon => Span::styled(":", self.style.text),
                Token::SemiColon => Span::styled(";", self.style.text),
                Token::Slash => Span::styled("/", self.style.text),

                _ => Span::from(format!("TODO: {}", self.token.to_string())),
            });

            // info!("Spans {:#?}", spans);

            self.read_token();
        }
        self.read_token();
        let line = Line::from(spans);
        // info!("Line {:#?}", line);
        Ok(line)
    }

    fn read_token(&mut self) {
        if self.read_position >= self.input.len() {
            self.token = Token::EOF;
        } else {
            self.token = self.input[self.read_position].clone()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&mut self) -> Token {
        if self.read_position >= self.input.len() {
            return Token::EOF;
        } else {
            self.input[self.read_position].clone()
        }
    }
}

#[cfg(test)]
mod test {

    use crate::parser::lexer::Lexer;
    use anyhow::{Ok, Result};

    use super::Parser;

    #[test]
    fn test_expr() -> Result<()> {
        pretty_env_logger::init();
        let md = "## test test 123 -
Lol 
- 1 
* 2,
*
*
abc

2
";

        let mut lexer = Lexer::new();
        let res = lexer.parse::<&str>(&md)?;

        let mut parser = Parser::new(res, None);
        let res = parser.parse()?;
        //println!("{:#?}", res);
        let a = res.iter().map(|f| {
            f.spans
                .iter()
                .map(|f| format!("{}", f.content.to_string()))
                .collect::<Vec<String>>().join("")
        }).collect::<String>();
        println!("{:?}",a);

        assert_eq!(true, true);
        Ok(())
    }
}
