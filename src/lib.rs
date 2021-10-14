use std::io;
use std::error::Error;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;
use events::{Event, Events};
use game::Board;

pub mod ui;
pub mod events;
pub mod game;
pub mod flags;

pub fn run(f: flags::Flags) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let events = Events::new(f.tick);

    let mut board = Board::new();

    loop {
        terminal.draw(|f| ui::draw(f, &board))?;

        match events.get_event()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                },
                Key::Right => {
                    board.move_right();
                },
                Key::Left => {
                    board.move_left();
                },
                Key::Down => {
                    board.move_down();
                },
                Key::Char(' ') => {
                    board.rotate(true);
                },
                Key::Char('x') => {
                    board.rotate(false);
                },
                _ => {
                    // process keys here
                },
            },
            Event::Tick => {
                // if tetronimo fell to the end
                if board.tick() {
                    loop {
                        let delete_row = board.can_delete();
                        if delete_row == -1 {
                            break;
                        }

                        board.delete(delete_row);
                        terminal.draw(|f| ui::draw(f, &board))?;
                        board.collapse(delete_row);
                        terminal.draw(|f| ui::draw(f, &board))?;
                    }
                }
                // process tick here
            },
        }
    }

    Ok(())
}
