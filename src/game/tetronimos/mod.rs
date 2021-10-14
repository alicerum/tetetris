use std::collections::HashMap;
use tui::style::Color;
use rand::{thread_rng, Rng};

mod tables;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i8,
    pub y: i8,

    pub c: Color,
}

#[derive(Copy, Clone)]
pub enum Type {
    I,
    L,
    T,
    S,
    O,
}

impl Type {
    // fill_bag fills the bag of enum types
    fn fill_bag() -> [Type; 5] {
        [Type::I, Type::L, Type::T, Type::S, Type::O]
    }
}

pub struct TetronimoBag {
    types: [Type; 5],
    size: u8,
}

impl TetronimoBag {
    pub fn new() -> TetronimoBag {
        TetronimoBag{types: Type::fill_bag(), size: 5}
    }

    pub fn draw_next(&mut self) -> Type {
        let index = thread_rng().gen_range(0..self.size);
        let result = self.types[index as usize];
        // move it all around
        for i in index..self.size-1 {
            self.types[i as usize] = self.types[(i+1) as usize];
        }
        self.size -= 1;
        if self.size == 0 {
            self.types = Type::fill_bag();
            self.size = 5;
        }
        result
    }
}

pub struct Tetronimo {
    pub pixels: [Pixel; 4],
}

impl Tetronimo {
    pub fn new (t: Type) -> Tetronimo {
        Tetronimo {
            pixels: tables::fill_new_pixels(t),
        }
    }

    // move_tick returns false in case we cannot move tetronimo anymore
    pub fn check_pixel(&self, x: i8, y: i8) -> Option<Color> {
        for p in &self.pixels {
            if p.x == x && p.y == y {
                return Some(p.c);
            }
        }

        None
    }

    pub fn move_offset(&mut self, offset: (i8, i8), board: &HashMap<(i8, i8), Color>) -> bool {
        for p in &mut self.pixels {
            let (new_x, new_y) = (p.x + offset.0, p.y + offset.1);
            if new_y == 20 ||
                new_x == 10 || new_x == -1 ||
                board.get(&(new_x, new_y)).is_some() {

                return false;
            }
        }

        self.pixels.iter_mut().for_each(|p| {
            p.x += offset.0;
            p.y += offset.1;
        });
        true
    }

    // rotate is where all our funny map happens!
    // we rotate tetronimo around rotation point
    // (0th element of pixel array)
    pub fn rotate(&mut self, clockwise: bool) {
        for i in 1..4 {
            let relative_x = self.pixels[i].x - self.pixels[0].x;
            let relative_y = self.pixels[0].y - self.pixels[i].y;

            if clockwise {
                self.pixels[i].x = self.pixels[0].x + relative_y;
                self.pixels[i].y = self.pixels[0].y + relative_x;
            } else {
                self.pixels[i].x = self.pixels[0].x - relative_y;
                self.pixels[i].y = self.pixels[0].y - relative_x;
            }
        }
    }
}
