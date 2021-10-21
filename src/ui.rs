use crate::game::Board;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::Frame;

const HELP_LEN: usize = 7;
const HELP_LINES: [&str; HELP_LEN] = [
    "HELP:",
    "Directional keys: ← → ↓",
    "Rotate clockwise: ↑ space x",
    "Rotate counterclockwize: z",
    "Hard drop: ret",
    "Pause: esc",
    "Quit: q C-c",
];

pub fn draw<B: Backend>(f: &mut Frame<B>, game_board: &Board) {
    let term_rect = f.size();

    let mut cell_height = 2;
    let mut cell_width = 4;

    if cell_height * 20 + 2 > term_rect.height || cell_width * 10 + 2 > term_rect.width {
        cell_height = 1;
        cell_width = 2;
    }

    if cell_height * 20 + 2 > term_rect.height || cell_width * 10 + 2 > term_rect.width {
        // terminal is still too small, please resize
        let b = Block::default().title(Span::styled(
            "Terminal is too small, please resize!",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));

        f.render_widget(b, term_rect);

        return;
    }

    let board_height = 20 * cell_height + 2;
    let board_width = 10 * cell_width + 2;

    let vpadding = (term_rect.height - board_height) / 2;
    let hpadding = (term_rect.width - board_width) / 2;

    let vouter = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(vpadding),
            Constraint::Length(board_height),
            Constraint::Min(vpadding),
        ])
        .split(term_rect);

    let houter = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(hpadding),
            Constraint::Length(board_width),
            Constraint::Min(hpadding),
        ])
        .split(vouter[1]);

    let left_pad = houter[0];
    let right_pad = houter[2];

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        " TeTetris ",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ));
    f.render_widget(block, houter[1]);

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
        .split(houter[1]);

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

    if left_pad.width > 0 {
        let constraints = [Constraint::Length(HELP_LEN as u16), Constraint::Min(0)];

        let help_block = Layout::default()
            .constraints(constraints)
            .direction(Direction::Vertical)
            .margin(1)
            .split(left_pad)[0];

        let lines = Layout::default()
            .constraints([Constraint::Length(1); HELP_LEN])
            .direction(Direction::Vertical)
            .split(help_block);

        for i in 0..HELP_LEN {
            let msg_len = HELP_LINES[i].chars().count();
            let msg_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(msg_len as u16)])
                .split(lines[i])[1];

            let b = Block::default().title(HELP_LINES[i]);
            f.render_widget(b, msg_rect);
        }
    }

    if right_pad.width > 0 {
        let right_info = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Min(0),
            ])
            .split(right_pad);

        let score_block = Block::default().title(Span::styled(
            format!("Score: {}", game_board.score()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        f.render_widget(score_block, right_info[0]);

        if let Some(ps) = game_board.upcoming_pixels() {
            let upcoming_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(8)])
                .split(right_info[1])[0];
            let cols = vec![Constraint::Length(2); 5];
            let rows = vec![Constraint::Length(1); 4];

            let ccols = Layout::default()
                .constraints(cols)
                .direction(Direction::Horizontal)
                .split(upcoming_rect);
            for col in 0..4 {
                let crows = Layout::default()
                    .constraints(rows.clone())
                    .direction(Direction::Vertical)
                    .split(ccols[col]);

                for row in 0..4 {
                    let cell = crows[row];
                    let p_x = (col as i8) + 3;
                    let p_y = (row as i8) - 4;

                    for i in 0..4 {
                        if ps[i].x == p_x && ps[i].y == p_y {
                            let b = Block::default().style(Style::default().bg(ps[i].c));
                            f.render_widget(b, cell);
                        }
                    }
                }
            }
        }
    }

    // Now, if game has ended or is paused,
    // it is time to render end game message on top
    if game_board.is_game_over() || game_board.is_paused() {
        let msg_color;
        let msg;
        if game_board.is_game_over() {
            msg = "! GAME  OVER !";
            msg_color = Color::Red;
        } else {
            msg = "    PAUSED    ";
            msg_color = Color::Green;
        };

        let r = houter[1];
        let cs = [
            Constraint::Length(5 * cell_height),
            Constraint::Length(5),
            Constraint::Min(0),
        ];

        let vmsg = Layout::default()
            .direction(Direction::Vertical)
            .constraints(cs)
            .split(r)[1];

        let block_width = msg.chars().count() + 4;
        let pad = (r.width - block_width as u16) / 2;
        let hmsg = Layout::default()
            .constraints([
                Constraint::Length(pad),
                Constraint::Length(block_width as u16),
                Constraint::Length(pad),
            ])
            .direction(Direction::Horizontal)
            .split(vmsg)[1];

        let b = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        f.render_widget(b, hmsg);

        let title_widget = Block::default().title(Span::styled(
            msg,
            Style::default().fg(msg_color).add_modifier(Modifier::BOLD),
        ));

        let msg = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(2)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(hmsg)[1];

        f.render_widget(title_widget, msg);
    }
}
