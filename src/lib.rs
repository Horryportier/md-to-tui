//// # Usage
//// this library implements `MarkdownParsable` for types that implement `ToString` trait.
//// You can use `parse_markdown` fn to parse markdown to `Text`.
//// `parse_markdown` takes `option` of `MdStyle` and returns `Result<Text<'static>, Error>`
//// ```rust
//// example
//// let md = "
//// # TODO
////
//// - [ ] one
//// - [ ] two
////
//// [link](http://exp.com)
////
//// "
//// let res = md.parse_markdown(Some(style))
//// `
use error::Error;
use parser::parser::Parser;
use ratatui::text::Text;
use style::style::MdStyle;
mod error;
mod parser;
pub mod style;

/// trait MarkdownParsable will take any trait that impl `ToString` and parse it into ratatui Text
pub trait MarkdownParsable {
    /// Convert type to Text
    fn parse_markdown(&self, style: Option<MdStyle>) -> Result<Text<'static>, Error>;
}

impl<T> MarkdownParsable for T
where
    T: ToString,
{
    fn parse_markdown(&self, style: Option<MdStyle>) -> Result<Text<'static>, Error> {
        let input = self.to_string();
        let parser = Parser::new(input, style);
        let res = parser.parse();
        match res {
            Err(err) => Ok(Text::from(format!("{}", err))),
            Ok(res) => Ok(Text::from(res)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        error::Error,
        fs, io,
        time::{Duration, Instant},
    };

    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        style::{Color, Modifier, Style},
        text::Span,
        widgets::{Block, Borders, Paragraph},
        Frame, Terminal,
    };

    struct App {
        scroll: u16,
    }

    impl App {
        fn new() -> App {
            App { scroll: 0 }
        }

        fn on_tick(&mut self) {
            self.scroll += 1;
            self.scroll %= 10;
        }
    }
    #[test]
    #[ignore = "github action can't run tui"]
    fn ui_test() -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new();
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{err:?}");
        }

        assert_eq!(true, true);
        Ok(())
    }

    fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        mut app: App,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui(f, &app))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    #[allow(dead_code)]
    fn ui<B: Backend>(f: &mut Frame<B>, _app: &App) {
        let size = f.size();

        // Words made "loooong" to demonstrate line breaking.
        let _file = fs::read("src/test/test.md").unwrap();
        let s = "
---
__Advertisement :)__

- [pica](https://nodeca.github.io/pica/demo/)  high quality and fast image
  resize in browser.
- __[babelfish](https://github.com/nodeca/babelfish/)__ - developer friendly
  i18n with plurals support and easy syntax.

*You will like those projects!*
---

# h1 Heading 8-)
## h2 Heading
### h3 Heading
#### h4 Heading
##### h5 Heading
###### h6 Heading

<table>
    <tr>
        <td>Foo</td>
    </tr>
</table>
";
        let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
        long_line.push('\n');

        let block = Block::default().style(Style::default().fg(Color::Black));
        f.render_widget(block, size);

        // let text = match String::from_utf8(file).unwrap().parse_markdown(None) {
        //     Ok(text) => text,
        //     Err(err) => Text::from(err.to_string()),
        // };
        let text = match s.parse_markdown(None) {
            Ok(text) => text,
            Err(err) => Text::from(err.to_string()),
        };

        let create_block = |title| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray))
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
        };

        let paragraph = Paragraph::new(text.clone())
            .style(Style::default().fg(Color::Gray))
            .block(create_block("Default alignment (Left), with wrap"));
        // .wrap(Wrap { trim: true });
        f.render_widget(paragraph, f.size());
    }
}
