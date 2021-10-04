use std::io;
use std::error::Error;

use tui::Terminal;
use tui::backend::TermionBackend;

use termion::raw::IntoRawMode;
use termion::event::Key;

use events::{Event, Events};

mod ui;
mod events;

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| ui::draw(f))?;


        match events.get_event()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                },
                _ => {
                    // process keys here
                },
            },
            Event::Tick => {
                // nothing for now
            },
        }

    }

    Ok(())
}
