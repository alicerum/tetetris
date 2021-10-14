use std::collections::HashMap;
use tui::style::Color;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i8,
    pub y: i8,

    pub c: Color,
}

#[derive(Copy,Clone)]
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
            pixels: fill_new_pixels(t),
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
}

// we use tables for tetronimos spawn
// as the easiest way
fn fill_new_pixels(t: Type) -> [Pixel; 4] {
    let res = [Pixel{x: 0, y: 0, c: Color::White}; 4];
    match t {
        Type::I => fill_new_i(res),
        Type::L => fill_new_l(res),
        Type::T => fill_new_t(res),
        Type::O => fill_new_o(res),
        Type::S => fill_new_s(res),
    }
}

fn fill_new_i(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Cyan;

    ps[0] = Pixel{x: 4, y:-1, c: c};
    ps[1] = Pixel{x: 3, y:-1, c: c};
    ps[2] = Pixel{x: 5, y:-1, c: c};
    ps[3] = Pixel{x: 6, y:-1, c: c};

    ps
}

fn fill_new_l(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Yellow;

    ps[0] = Pixel{x: 4, y:-1, c: c};
    ps[1] = Pixel{x: 3, y:-1, c: c};
    ps[2] = Pixel{x: 5, y:-1, c: c};
    ps[3] = Pixel{x: 5, y:-2, c: c}; // upper

    ps
}

fn fill_new_t(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Magenta;

    ps[0] = Pixel{x: 4, y:-1, c: c};
    ps[1] = Pixel{x: 4, y:-2, c: c};
    ps[2] = Pixel{x: 3, y:-1, c: c};
    ps[3] = Pixel{x: 5, y:-1, c: c};

    ps
}

fn fill_new_s(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Green;

    ps[0] = Pixel{x: 4, y:-1, c: c};
    ps[1] = Pixel{x: 3, y:-1, c: c};
    ps[2] = Pixel{x: 4, y:-2, c: c};
    ps[3] = Pixel{x: 5, y:-2, c: c};

    ps
}

fn fill_new_o(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::LightYellow;

    ps[0] = Pixel{x: 4, y:-1, c: c};
    ps[1] = Pixel{x: 4, y:-2, c: c};
    ps[2] = Pixel{x: 5, y:-2, c: c};
    ps[3] = Pixel{x: 5, y:-1, c: c};

    ps
}
