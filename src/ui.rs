use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Block, Borders};

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(10)
        .constraints(
            [
                Constraint::Min(22),
                Constraint::Min(20),
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Tetris")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
