mod tetronimos;

use std::collections::HashMap;
use tetronimos::{Tetronimo, TetronimoBag, Type};
use tui::style::Color;

pub enum ScoreAction {
    RowCleared(u8),
    FallLength(u8),
    HardDrop(u8),
}

pub enum MoveDirection {
    Down,
    Left,
    Right,
}

pub struct Board {
    falling: Option<Tetronimo>,
    upcoming: Option<Tetronimo>,

    board: HashMap<(i8, i8), Color>,

    score: u64,
    game_over: bool,
    pause: bool,

    bag: TetronimoBag,
}

impl Board {
    pub fn new() -> Board {
        Board {
            falling: None,
            upcoming: None,

            board: HashMap::new(),
            score: 0,
            game_over: false,
            pause: false,

            bag: TetronimoBag::new(),
        }
    }

    pub fn move_tetrinomo(&mut self, direction: MoveDirection) {
        if self.is_paused() {
            return;
        }

        match direction {
            MoveDirection::Down => self.move_down(),
            MoveDirection::Left => self.move_left(),
            MoveDirection::Right => self.move_right(),
        }
    }

    fn move_left(&mut self) {
        if let Some(t) = &mut self.falling {
            t.move_offset((-1, 0), &self.board);
        }
    }

    fn move_right(&mut self) {
        if let Some(t) = &mut self.falling {
            t.move_offset((1, 0), &self.board);
        }
    }

    fn move_down(&mut self) {
        if let Some(t) = &mut self.falling {
            if t.move_offset((0, 1), &self.board) {
                t.inc_dropped();
            }
        }
    }

    pub fn rotate(&mut self, clockwise: bool) {
        if self.is_paused() {
            return;
        }

        if let Some(t) = &mut self.falling {
            t.rotate_and_kick(clockwise, &self.board);
        }
    }

    fn next_tetronimo_type(&mut self) -> Type {
        self.bag.draw_next()
    }

    // return true in case tetronimo was stuck
    pub fn tick(&mut self) -> bool {
        if self.game_over || self.pause {
            // if game over or paused, don't make anything fall
            // just ignore the ticks
            return false;
        }
        match &mut self.falling {
            Some(t) => {
                // if cannot move anymore
                if !t.move_offset((0, 1), &self.board) {
                    self.lock_piece();
                    return true;
                }
            }
            None => {
                if let Some(_) = self.upcoming {
                    self.falling = self.upcoming.take();
                } else {
                    self.falling = Some(Tetronimo::new(self.next_tetronimo_type()));
                }

                self.upcoming = Some(Tetronimo::new(self.next_tetronimo_type()));
            }
        }

        false
    }

    pub fn hard_drop(&mut self) {
        if let Some(t) = self.falling.as_mut() {
            let mut rows_dropped = 0;
            while t.move_offset((0, 1), &self.board) {
                rows_dropped += 1;
            }
            self.lock_piece();
            self.add_score(ScoreAction::HardDrop(rows_dropped));
        }
    }

    fn lock_piece(&mut self) {
        let t = self.falling.as_ref().unwrap();
        for p in t.pixels {
            if p.y < 0 {
                // piece couldn't move and part of it was above the screen
                // is how game over is determined for tetris
                self.game_over = true;
            }
            self.board.insert((p.x, p.y), p.c);
        }
        let fall = t.dropped();
        self.add_score(ScoreAction::FallLength(fall));
        self.falling = None;
    }

    pub fn check_pixel(&self, x: i8, y: i8) -> Option<Color> {
        if let Some(t) = &self.falling {
            if let Some(c) = t.check_pixel(x, y) {
                return Some(c);
            }
        }

        self.board.get(&(x, y)).copied()
    }

    pub fn can_delete(&self) -> Option<i8> {
        for row in (0..20).rev() {
            let mut all_filled = true;

            for col in 0..10 {
                if self.board.get(&(col, row)).is_none() {
                    all_filled = false;
                    break;
                }
            }

            if all_filled {
                return Some(row);
            }
        }
        None
    }

    pub fn delete(&mut self, row: i8) {
        for col in 0..10 {
            self.board.remove(&(col, row));
        }
    }

    pub fn collapse(&mut self, row: i8) {
        for row in (-2..row + 1).rev() {
            for col in 0..10 {
                if self.board.get(&(col, row - 1)).is_none() {
                    self.board.remove(&(col, row));
                } else {
                    self.board.insert((col, row), self.board[&(col, row - 1)]);
                }
            }
        }
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn add_score(&mut self, action: ScoreAction) {
        self.score += match action {
            ScoreAction::RowCleared(n) => match n {
                0 => 0,
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => 1200,
            },
            ScoreAction::FallLength(n) => n as u64,
            ScoreAction::HardDrop(n) => n as u64 * 2,
        }
    }

    pub fn upcoming_pixels(&self) -> Option<&[tetronimos::Pixel; 4]> {
        self.upcoming.as_ref().map(|t| &t.pixels)
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn is_paused(&self) -> bool {
        self.pause
    }
}
