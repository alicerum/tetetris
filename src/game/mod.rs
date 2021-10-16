mod tetronimos;

use std::collections::HashMap;
use tetronimos::{Tetronimo, Type, TetronimoBag};
use tui::style::Color;

pub enum ScoreAction {
    RowCleared(u8),
    FallLength(u8),
}

pub struct Board {
    falling: Option<Tetronimo>,
    board: HashMap<(i8, i8), Color>,

    score: u64,
    game_over: bool,

    bag: TetronimoBag,
}

impl Board {
    pub fn new() -> Board {
        Board{
            falling: None,
            board: HashMap::new(),
            score: 0,
            game_over: false,

            // TODO: rework the bag thing
            // right now, it uses multiple vectors and some uncomfortable
            // shuffling memory around, should be nicer and neater
            bag: TetronimoBag::new(),
        }
    }

    pub fn move_left(&mut self) {
        if let Some(t) = &mut self.falling {
            t.move_offset((-1, 0), &self.board);
        }
    }

    pub fn move_right(&mut self) {
        if let Some(t) = &mut self.falling {
            t.move_offset((1, 0), &self.board);
        }
    }

    pub fn move_down(&mut self) {
        if let Some(t) = &mut self.falling {
            if t.move_offset((0, 1), &self.board) {
                t.inc_dropped();
            }
        }
    }

    pub fn rotate(&mut self, clockwise: bool) {
        if let Some(t) = &mut self.falling {
            t.rotate_and_kick(clockwise, &self.board);
        }
    }

    fn next_tetronimo_type(&mut self) -> Type {
        self.bag.draw_next()
    }

    // return true in case tetronimo was stuck
    pub fn tick(&mut self) -> bool {
        if self.game_over {
            // TODO: draw game over text and maybe final score
            // on the screen in case of game over
            return false;
        }
        match &mut self.falling {
            Some(t) => {
                // if cannot move anymore
                if !t.move_offset((0, 1), &self.board) {
                    self.lock_piece();
                    return true;
                }
            },
            None => {
                self.falling = Some(Tetronimo::new(self.next_tetronimo_type()));
            },
        }

        false
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

    pub fn can_delete(&self) -> i8 {
        for row in (0..20).rev() {
            let mut all_filled = true;

            for col in 0..10 {
                if self.board.get(&(col, row)).is_none() {
                    all_filled = false;
                    break;
                }
            }

            if all_filled {
                return row;
            }
        }
        -1
    }

    pub fn delete(&mut self, row: i8) {
        for col in 0..10 {
            self.board.remove(&(col, row));
        }
    }

    pub fn collapse(&mut self, row: i8) {
        for row in (-2..row+1).rev() {
            for col in 0..10 {
                if self.board.get(&(col, row-1)).is_none() {
                    self.board.remove(&(col, row));
                } else {
                    self.board.insert((col,row), self.board[&(col, row-1)]);
                }
            }
        }
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn add_score(&mut self, action: ScoreAction) {
        self.score += match action {
            ScoreAction::RowCleared(n) => {
                match n {
                    0 => 0,
                    1 => 40,
                    2 => 100,
                    3 => 300,
                    4 => 1200,
                    _ => 1200,
                }
            },
            ScoreAction::FallLength(n) => n as u64,
        }
    }
}
