use ratatui::style::{Style, Color, Modifier};


#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct MdStyle {
  pub heading: Style,
  pub h1: Style,
  pub h2: Style,
  pub h3: Style,
  pub h4: Style,
  pub h5: Style,
  pub h6: Style,
  pub underline_heading: Style,

  pub list: Style,

  pub text: Style,
  pub paragram: Style,

  pub bold: Style,
  pub italic: Style, 
  pub backtick: Style,
  pub blocqoutes: Style,
  pub horizontal_rule: Style,
  pub colored: Style,

  pub link: Style,
  pub link_text: Style,
  pub tag: Style,

  pub code: Style,
}

impl Default for MdStyle {
    fn default() -> Self {
        MdStyle { 
        heading: Style::default().fg(Color::Cyan),
        h1: Style::default().fg(Color::Magenta),
        h2: Style::default().fg(Color::Magenta),
        h3: Style::default().fg(Color::LightYellow),
        h4: Style::default().fg(Color::LightYellow),
        h5: Style::default().fg(Color::LightGreen),
        h6: Style::default().fg(Color::LightCyan),

        underline_heading: Style::default().fg(Color::LightGreen),
        list: Style::default().fg(Color::LightRed),
        paragram: Style::default().fg(Color::DarkGray),
        text: Style::default().fg(Color::White),

        bold: Style::default().add_modifier(Modifier::BOLD),
        italic: Style::default().add_modifier(Modifier::ITALIC),
        backtick: Style::default().fg(Color::Gray).bg(Color::Black),
        blocqoutes: Style::default().fg(Color::Gray).bg(Color::Black),
        horizontal_rule: Style::default().fg(Color::Gray).bg(Color::Red),
        colored: Style::default().bg(Color::Yellow),

        link: Style::default().fg(Color::Blue),
        link_text:  Style::default().fg(Color::Red),
        tag: Style::default().bg(Color::Cyan),
        
        code: Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
        }
    }
}

