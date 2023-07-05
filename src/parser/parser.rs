use std::sync::{Arc, Mutex};

use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::{error::Error, style::style::MdStyle};

use comrak::{
    self,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

use super::parser_helpers::{
    generate_blockquete, generate_code, generate_codeblock, generate_heading,
    generate_indent, generate_link, generate_list, generate_rule, generate_description_item, generate_html_block,
};

#[derive(Debug, Clone)]
pub struct Parser {
    pub input: String,
    pub style: MdStyle,
}

impl Parser {
    pub fn new(input: String, style: Option<MdStyle>) -> Parser {
        Parser {
            input,
            style: match style {
                None => MdStyle::default(),
                Some(style) => style, 
            },
        }
    }

    pub fn parse(&self) -> Result<Vec<Line<'static>>, Error> {
        let mut lines: Vec<Line> = vec![];
        for line in self.input.clone().split("\n") {
            let res = self.parse_line(line.into())?;
            lines.push(res.into());
        }
        Ok(lines)
    }

    pub fn parse_line(&self, line: String) -> Result<Vec<Span<'static>>, Error> {
        let style = &self.style;

        let arena = Arena::new();
        let ast = parse_document(&arena, &line, &ComrakOptions::default());

        let spans = Arc::new(Mutex::new(Vec::<Span>::new()));
        iter_nodes(ast, &|node| {
            let parent = match &node.clone().parent() {
              None => NodeValue::Document,  
              Some(node) => node.clone().data.borrow_mut().value.to_owned(),
            };
            let val = &node.clone().data.borrow_mut().value.to_owned();

            match &mut node.data.borrow_mut().value {
                NodeValue::Document => false,
                NodeValue::Paragraph => false,
                NodeValue::Strong => false,
                NodeValue::Emph => false,
                NodeValue::DescriptionList => false,
                NodeValue::DescriptionTerm => false,
                NodeValue::DescriptionDetails => false,
                NodeValue::HtmlBlock(html) => {
                    spans.lock().unwrap().push(generate_html_block(html, parent, style));
                    false
                }
                NodeValue::HtmlInline(text) => {
                    spans.lock().unwrap().push(generate_indent(text, parent, style));
                    false
                }
                NodeValue::FootnoteReference(text) => {
                    spans.lock().unwrap().push(generate_indent(text, parent, style));
                    false
                }
                NodeValue::FootnoteDefinition(text) => {
                    spans.lock().unwrap().push(generate_indent(text, parent, style));
                    false
                }
                &mut NodeValue::DescriptionItem(ref mut item) => {
                    spans.lock().unwrap().push(generate_description_item(item,parent, style));
                    false
                }
                &mut NodeValue::Heading(ref mut heading) => {
                    spans.lock().unwrap().push(generate_heading(heading, style));
                    false
                }
                &mut NodeValue::Text(ref mut text) => {
                    spans.lock().unwrap().push(generate_indent(text, parent, style));
                    false
                }
                &mut NodeValue::List(ref mut list) => {
                    spans.lock().unwrap().push(generate_list(list, parent, style));
                    false
                }
                &mut NodeValue::Item(ref mut list) => {
                    spans.lock().unwrap().push(generate_list(list, parent, style));
                    false
                }
                &mut NodeValue::BlockQuote => {
                    spans.lock().unwrap().push(generate_blockquete(style));
                    false
                }
                &mut NodeValue::LineBreak => true,
                &mut NodeValue::Code(ref mut code) => {
                    spans.lock().unwrap().push(generate_code(code, style));
                    false
                }
                &mut NodeValue::CodeBlock(ref mut codeblock) => {
                    spans
                        .lock()
                        .unwrap()
                        .push(generate_codeblock(codeblock, style));
                    false
                }
                &mut NodeValue::Link(ref mut link) => {
                    spans
                        .lock()
                        .unwrap()
                        .append(&mut generate_link(link, style));
                    false
                }
                NodeValue::ThematicBreak => {
                    spans.lock().unwrap().push(generate_rule(style));
                    false
                }
                NodeValue::FrontMatter(ref mut front_matter) => {
                    spans.lock().unwrap().push(Span::from(front_matter.to_owned()));
                    false
                }
                
                _ => {
                    spans.lock().unwrap().push(Span::styled(
                        format!("TODO: {:?}", val),
                        Style::default().fg(ratatui::style::Color::Red),
                    ));
                    false
                }
            }
        });

        Ok(spans.clone().lock().unwrap().to_vec())
    }
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F) -> bool
where
    F: Fn(&'a AstNode<'a>) -> bool,
{
    f(node);
    for c in node.children() {
        let is_end = iter_nodes(c, f);
        if is_end {
            break;
        }
    }
    true
}

#[cfg(test)]
mod test {

    use crate::parser::parser::Parser;
    use anyhow::{Ok, Result};

    #[test]
    #[ignore = "xd"]
    fn test_check_ast() -> Result<()> {
        let text = "
# A 
## A 
### A 
#### A 
##### A 
###### A 
--- 
=== 
---
+++
#ABC
`ABC`
*ABC*
**ABC**
***ABC***
> abc
> > **ABC***
> > >
# A
---
=== 
---
- [x] ABC
- [ ] ABC
    1. ABC
    1. ABC

";

        let parser = Parser::new(text.into(), None);

        let res = parser.parse()?;
        // assert_eq!(res, " ");

        Ok(())
    }
}
