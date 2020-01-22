mod emoji;
mod util;
use std::io::{self, Write};

use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

use crate::util::event::{Event, Events};

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    index: i32,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            index: 0,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let mut events = Events::new();

    // Create default app state
    let mut app = App::default();

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .render(&mut f, chunks[1]);
            let res = emoji::search(app.input.to_string());
            let messages = res.iter().enumerate().map(|(i, e)| {
                Text::styled(
                    format!("{}: {} {}", i, e.icon, e.name),
                    Style::default().fg(if app.index == i as i32 {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                )
            });
            List::new(messages)
                .block(Block::default().borders(Borders::ALL).title("Messages"))
                .render(&mut f, chunks[2]);
        })?;

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.width() as u16, 5)
        )?;
        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();

        // Handle input
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('\n') => {
                    app.messages.push(app.input.drain(..).collect());
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                Key::Up => {
                    if app.index != 0 {
                        app.index -= 1;
                    }
                }
                Key::Down => {
                    if app.index != 10 {
                        app.index += 1;
                    }
                }
                Key::Esc => {
                    events.enable_exit_key();
                    break;
                }
                _ => {}
            },

            _ => {}
        }
    }
    Ok(())
}
