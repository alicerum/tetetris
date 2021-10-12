pub mod tetronimos;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use tetronimos::{Tetronimo, Type};
use tui::style::Color;

pub struct Pixel {
    x: i8,
    y: i8,

    c: Color,
}

pub struct Board {
    falling: Option<Tetronimo>,
    board: HashMap<(i8, i8), Color>,

    game_over: bool,

    bag: Vec<Type>,
}

impl Board {
    pub fn new() -> Board {
        Board{
            falling: None,
            board: HashMap::new(),
            game_over: false,

            // TODO: rework the bag thing
            // right now, it uses multiple vectors and some uncomfortable
            // shuffling memory around, should be nicer and neater
            bag: Vec::new(),
        }
    }

    fn next_tetronimo_type(&mut self) -> Type {
        match self.bag.pop() {
            Some(t) => t,
            None => {
                self.bag = Type::fill_bag();
                let mut rng = rand::thread_rng();
                self.bag.shuffle(&mut rng);

                return self.bag.pop().unwrap();
            }
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            // TODO: draw game over text and maybe final score
            // on the screen in case of game over
            return;
        }
        match &mut self.falling {
            Some(t) => {
                // if cannot move anymore
                if !t.move_tick(&self.board) {
                    for p in &t.pixels {
                        if p.y < 0 {
                            // piece couldn't move and part of it was above the screen
                            // is how game over is determined for tetris
                            self.game_over = true; 
                        }
                        self.board.insert((p.x, p.y), p.c);
                    }
                    self.falling = None;
                }
            },
            None => {
                self.falling = Some(Tetronimo::new(self.next_tetronimo_type()));
            },
        }
    }

    pub fn check_pixel(&self, x: i8, y: i8) -> Option<Color> {
        if let Some(t) = &self.falling {
            if let Some(c) = t.check_pixel(x, y) {
                return Some(c);
            }
        }

        self.board.get(&(x, y)).copied()
    }
}
