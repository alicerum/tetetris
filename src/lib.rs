use events::{Event, Events};
use game::{Board, MoveDirection, ScoreAction};
use std::error::Error;
use std::io;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::Terminal;

mod events;
pub mod flags;
mod game;
mod ui;

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
                }
                Key::Right => {
                    board.move_tetrinomo(MoveDirection::Right);
                }
                Key::Left => {
                    board.move_tetrinomo(MoveDirection::Left);
                }
                Key::Down => {
                    board.move_tetrinomo(MoveDirection::Down);
                }
                Key::Char(' ') | Key::Char('x') | Key::Up => {
                    board.rotate(true);
                }
                Key::Char('\n') => {
                    board.hard_drop();

                    // we should check whether we can delete rows
                    // after hard drop, too
                    check_rows(&mut terminal, &mut board)?;
                }
                Key::Esc => board.toggle_pause(),
                Key::Char('z') => {
                    board.rotate(false);
                }
                _ => {
                    // nothing to do here
                }
            },
            Event::Tick => {
                // if tetronimo fell to the end
                if board.tick() {
                    check_rows(&mut terminal, &mut board)?;
                }
                // process tick here
            }
        }
    }

    Ok(())
}

fn check_rows<B: Backend>(
    terminal: &mut Terminal<B>,
    board: &mut Board,
) -> Result<(), Box<dyn Error>> {
    let mut amount_deleted = 0;
    loop {
        let delete_row = match board.can_delete() {
            Some(r) => r,
            None => break,
        };
        amount_deleted += 1;

        board.delete(delete_row);
        terminal.draw(|f| ui::draw(f, &board))?;
        thread::sleep(Duration::from_millis(20));
        board.collapse(delete_row);
        terminal.draw(|f| ui::draw(f, &board))?;
        thread::sleep(Duration::from_millis(20));
    }
    board.add_score(ScoreAction::RowCleared(amount_deleted));

    Ok(())
}
