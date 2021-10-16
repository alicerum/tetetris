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
    J,
    Z,
}

pub struct TetronimoBag {
    types: [Type; 7],
    size: u8,
}

impl TetronimoBag {
    pub fn new() -> TetronimoBag {
        TetronimoBag {
            types: TetronimoBag::fill_bag(),
            size: 7,
        }
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
            self.types = TetronimoBag::fill_bag();
            self.size = 7;
        }
        result
    }

    fn fill_bag() -> [Type; 7] {
        [Type::I, Type::L, Type::T, Type::S, Type::O, Type::J, Type::Z]
    }
}

pub struct Tetronimo {
    pub pixels: [Pixel; 4],

    dropped: u8,

    t: Type,
    rotation: i8,
}

impl Tetronimo {
    pub fn new (t: Type) -> Tetronimo {
        Tetronimo {
            pixels: tables::fill_new_pixels(t),
            dropped: 0,
            t: t,
            rotation: 0,
        }
    }

    pub fn inc_dropped(&mut self) {
        self.dropped += 1;
    }

    pub fn dropped(&self) -> u8 {
        self.dropped
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
    fn rotate(&mut self, clockwise: bool) {
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

        self.rotation += if clockwise {1} else {-1};
        if self.rotation == -1 {self.rotation = 3}
        if self.rotation == 4 {self.rotation = 0}
    }

    pub fn rotate_and_kick(&mut self, clockwise: bool, board: &HashMap<(i8, i8), Color>) {
        let previous_rotation = self.rotation;
        self.rotate(clockwise);

        let offsets = tables::get_kick_offsets(previous_rotation, self.rotation, self.t);
        for o in offsets {
            let mut collides = false;
            for p in self.pixels {
                let new_x = p.x + o.0;
                let new_y = p.y - o.1;

                if new_x < 0 || new_x > 9 || new_y > 19 ||
                    board.get(&(new_x, new_y)).is_some() {

                        collides = true;
                        break;
                }
            }

            if !collides {
                self.pixels.iter_mut().for_each(|p| {
                    p.x += o.0;
                    p.y -= o.1;
                });
                // we are completely done here
                return;
            }
        }

        // we couldn't find any good kick for us, time to give up
        self.rotate(!clockwise);
    }
}
