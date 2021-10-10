pub mod tetronimos;

use tetronimos::{Tetronimo, Type};
use tui::style::Color;

pub struct Pixel {
    x: usize,
    y: usize,

    c: Color,
}

pub struct Board {
    falling: Option<Tetronimo>,
    board: Vec<Pixel>,
}

impl Board {
    pub fn new() -> Board {
        Board{
            falling: Some(Tetronimo::new(Type::T)),
            board: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        match &mut self.falling {
            Some(t) => t.move_tick(),
            None => {
                // TODO: spawn new tetronimo
            },
        }
    }

    pub fn check_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if let Some(t) = &self.falling {
            if let Some(c) = t.check_pixel(x, y) {
                return Some(c);
            }
        }

        for p in &self.board {
            if p.x == x && p.y == y {
                return Some(p.c);
            }
        }

        None
    }
}
