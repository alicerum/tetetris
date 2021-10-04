use tui::backend::Backend;
use tui::Frame;
use tui::widgets::{Block, Borders};

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Block");
    f.render_widget(block, size);
}
