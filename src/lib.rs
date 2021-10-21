use std::io;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use events::{Event, Events};
use game::{Board, MoveDirection, ScoreAction};
use help::Help;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::Terminal;

mod events;
pub mod flags;
mod game;
mod help;
pub mod settings;
mod ui;

pub fn run(f: flags::Flags, s: settings::Settings) -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let events = Events::new(f.tick);
    let help = Help::from(&s.key_bindings);

    let mut board = Board::new();

    loop {
        terminal.draw(|f| ui::draw(f, &board, &help))?;

        match events.get_event()? {
            Event::Input(key) => {
                if matches!(key, Key::Char('q') | Key::Ctrl('c')) {
                    break;
                } else if s.key_bindings.move_right.contains(&key) {
                    board.move_tetrinomo(MoveDirection::Right);
                } else if s.key_bindings.move_left.contains(&key) {
                    board.move_tetrinomo(MoveDirection::Left);
                } else if s.key_bindings.move_down.contains(&key) {
                    board.move_tetrinomo(MoveDirection::Down);
                } else if s.key_bindings.rotate_cw.contains(&key) {
                    board.rotate(true);
                } else if s.key_bindings.rotate_ccw.contains(&key) {
                    board.rotate(false);
                } else if s.key_bindings.drop.contains(&key) {
                    board.hard_drop();

                    // we should check whether we can delete rows
                    // after hard drop, too
                    check_rows(&mut terminal, &mut board, &help)?;
                } else if s.key_bindings.pause.contains(&key) {
                    board.toggle_pause();
                }
            }
            Event::Tick => {
                // if tetronimo fell to the end
                if board.tick() {
                    check_rows(&mut terminal, &mut board, &help)?;
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
    help: &Help,
) -> Result<()> {
    let mut amount_deleted = 0;
    loop {
        let delete_row = match board.can_delete() {
            Some(r) => r,
            None => break,
        };
        amount_deleted += 1;

        board.delete(delete_row);
        terminal.draw(|f| ui::draw(f, &board, &help))?;
        thread::sleep(Duration::from_millis(20));
        board.collapse(delete_row);
        terminal.draw(|f| ui::draw(f, &board, &help))?;
        thread::sleep(Duration::from_millis(20));
    }
    board.add_score(ScoreAction::RowCleared(amount_deleted));

    Ok(())
}
