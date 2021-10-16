use std::io;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::event::Key;
use events::{Event, Events};
use game::{Board, ScoreAction, MoveDirection};

mod ui;
mod events;
mod game;
pub mod flags;

pub fn run(f: flags::Flags) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
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
                    board.move_tetrinomo(MoveDirection::Right);
                },
                Key::Left => {
                    board.move_tetrinomo(MoveDirection::Left);
                },
                Key::Down => {
                    board.move_tetrinomo(MoveDirection::Down);
                },
                Key::Char(' ') | Key::Char('x') | Key::Up => {
                    board.rotate(true);
                },
                Key::Esc => board.toggle_pause(),
                Key::Char('z') => {
                    board.rotate(false);
                },
                _ => {
                    // nothing to do here
                },
            },
            Event::Tick => {
                // if tetronimo fell to the end
                if board.tick() {
                    let mut amount_deleted = 0;
                    loop {
                        let delete_row = board.can_delete();
                        if delete_row == -1 {
                            break;
                        }
                        amount_deleted += 1;

                        board.delete(delete_row);
                        terminal.draw(|f| ui::draw(f, &board))?;
                        thread::sleep(Duration::from_millis(20));
                        board.collapse(delete_row);
                        terminal.draw(|f| ui::draw(f, &board))?;
                        thread::sleep(Duration::from_millis(20));
                    }
                    board.add_score(ScoreAction::RowCleared(amount_deleted));
                }
                // process tick here
            },
        }
    }

    Ok(())
}
