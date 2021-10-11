use crate::game::Board;
use tui::backend::Backend;
use tui::Frame;
use tui::text::Span;
use tui::style::{Style, Color, Modifier};
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Block, Borders, BorderType};

pub fn draw<B: Backend>(f: &mut Frame<B>, game_board: &Board) {

    let cell_height = 2;
    let cell_width = 4;

    let board_height = 20 * cell_height + 4;
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

    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, outer[1]);

    let mut vcs = Vec::new();
    let mut hcs = Vec::new();
    for _ in 0..10 {
        vcs.push(Constraint::Length(cell_width));
    }
    for _ in 0..20 {
        hcs.push(Constraint::Length(cell_height));
    }
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vcs)
        .split(outer[1]);

    for i in 0..10 {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(hcs.clone())
            .split(cols[i as usize]);

        for j in 0..20 {
            if let Some(c) = game_board.check_pixel(i, j) {
                let mut cell_block = Block::default();
                cell_block = cell_block.style(Style::default().bg(c));
                f.render_widget(cell_block, rows[j as usize]);
            }
        }
    }
}
