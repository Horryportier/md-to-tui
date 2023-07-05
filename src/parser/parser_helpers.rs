use comrak::nodes::{NodeCode, NodeCodeBlock, NodeHeading, NodeLink, NodeList, NodeValue, NodeDescriptionItem, NodeHtmlBlock};
use ratatui::text::Span;

use crate::style::style::MdStyle;

pub fn generate_heading(heading: &mut NodeHeading, style: &MdStyle) -> Span<'static> {
    match heading.level {
        1 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h1),
        2 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h2),
        3 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h3),
        4 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h4),
        5 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h5),
        6 => Span::styled(format!("{} ", "#".repeat(heading.level.into())), style.h6),
        _ => Span::styled(
            format!("{} ", "#".repeat(heading.level.into())),
            style.heading,
        ),
    }
}

pub fn generate_indent(indent: &mut String, parent: NodeValue, style: &MdStyle) -> Span<'static> {
    let mut span = Span::styled(indent.clone(), style.text);
    match  parent {
           NodeValue::Strong => span.patch_style(style.bold),
           NodeValue::Emph => span.patch_style(style.italic),
           NodeValue::CodeBlock(_) => span.patch_style(style.code),
           NodeValue::Code(_) => span.patch_style(style.code),
           NodeValue::Link(_) => span = Span::styled(format!("[{}]", indent), style.link_text),
           _ =>  {}
    }
    span
}

pub fn generate_list(list: &mut NodeList,parent: NodeValue, style: &MdStyle) -> Span<'static> {
    match parent {
     NodeValue::List(_) => Span::from(""),
     _ => {
        match list.list_type {
        comrak::nodes::ListType::Bullet => {
            Span::styled(format!("{}{}"," ".repeat(list.padding),   "•"), style.list)
        }
        comrak::nodes::ListType::Ordered => Span::styled(
            format!("{}{}. ", " ".repeat(list.padding),  list.start),
            style.list,
        )
        }
     }   
    }
}

pub fn generate_blockquete(style: &MdStyle) -> Span<'static> {
    Span::styled(">", style.blocqoutes)
}

pub fn generate_code(code: &mut NodeCode, style: &MdStyle) -> Span<'static> {
    Span::styled(
        format!(
            "{}{}{}",
            "`".repeat(code.num_backticks),
            code.literal,
            "`".repeat(code.num_backticks)
        ),
        style.backtick,
    )
}

pub fn generate_codeblock(code: &mut NodeCodeBlock, style: &MdStyle) -> Span<'static> {
    Span::styled(
        format!("{}{}", "`".repeat(3),  code.info),
        style.code,
    )
}
pub fn generate_link(link: &mut NodeLink, style: &MdStyle) -> Vec<Span<'static>> {
    vec![
        Span::styled(format!("({})", link.url), style.link),
        // Span::styled(format!("", link.title), style.link_text),
    ]
}

pub fn generate_rule(style: &MdStyle) -> Span<'static> {
    Span::styled("------", style.horizontal_rule)
}

pub fn generate_description_item(item: &mut NodeDescriptionItem, parent: NodeValue,  style: &MdStyle) -> Span<'static> {
    todo!()
}

pub fn generate_html_block(html: &mut NodeHtmlBlock, parent: NodeValue,  style: &MdStyle) -> Span<'static> {
       Span::styled(format!("{},{}", html.literal, html.block_type), style.code)
}
