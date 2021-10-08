use tui::backend::Backend;
use tui::Frame;
use tui::text::Span;
use tui::style::{Style, Color, Modifier};
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Block, Borders, BorderType};

pub fn draw<B: Backend>(f: &mut Frame<B>) {

    let cell_height = 2;
    let cell_width = 4;

    let board_height = 20 * cell_height + 2;
    let board_width = 10 * cell_width + 2;

    let term_rect = f.size();

    let b = Block::default()
        .title(
            Span::styled("TeTetris",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(b, term_rect);

    let vpadding = (term_rect.height - board_height) / 2;
    let hpadding = (term_rect.width - board_width) / 2;

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Min(vpadding),
            Constraint::Length(board_height),
            Constraint::Min(vpadding),
        ])
        .split(term_rect);

    let outer = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![
            Constraint::Min(hpadding),
            Constraint::Length(board_width),
            Constraint::Min(hpadding),
        ])
        .split(outer[1]);

    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, outer[1]);
}
