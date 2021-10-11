use std::collections::HashMap;
use tui::style::Color;
use super::Pixel;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum Type {
    I,
    L,
    T,
    S,
    O,
}

impl Type {
    // fill_bag fills the bag of enum types
    pub fn fill_bag() -> Vec<Type> {
        vec![Type::I, Type::L, Type::T, Type::S, Type::O]
    }
}

impl Distribution<Type> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
        match rng.gen_range(0..=4) {
            0 => Type::I,
            1 => Type::L,
            2 => Type::T,
            3 => Type::S,
            _ => Type::O,
        }
    }
}

pub struct Tetronimo {
    t: Type,

    pub pixels: Vec<Pixel>,
}

impl Tetronimo {
    pub fn new (ttype: Type) -> Tetronimo {
        let mut t = Tetronimo {
            t: ttype,
            pixels: Vec::new(),
        };

        match t.t {
            Type::I => fill_new_i(&mut t),
            Type::L => fill_new_l(&mut t),
            Type::T => fill_new_t(&mut t),
            Type::O => fill_new_o(&mut t),
            Type::S => fill_new_s(&mut t),
        }

        t
    }

    // move_tick returns false in case we cannot move tetronimo anymore
    pub fn move_tick(&mut self, board: &HashMap<(i8, i8), Color>) -> bool {
        for p in &self.pixels {
            if p.y == 19 {
                return false;
            }
            if let Some(_) = board.get(&(p.x, p.y + 1)) {
                return false;
            }
        }

        self.pixels.iter_mut().for_each(|p| {
            p.y += 1;
        });

        true
    }

    pub fn check_pixel(&self, x: i8, y: i8) -> Option<Color> {
        for p in &self.pixels {
            if p.x == x && p.y == y {
                return Some(p.c);
            }
        }

        None
    }
}

fn fill_new_i(t: &mut Tetronimo) {
    let c = Color::Cyan;

    t.pixels.push(Pixel{x: 3, y:-1, c: c});
    t.pixels.push(Pixel{x: 4, y:-1, c: c});
    t.pixels.push(Pixel{x: 5, y:-1, c: c});
    t.pixels.push(Pixel{x: 6, y:-1, c: c});
}

fn fill_new_l(t: &mut Tetronimo) {
    let c = Color::Yellow;

    t.pixels.push(Pixel{x: 3, y:-1, c: c});
    t.pixels.push(Pixel{x: 4, y:-1, c: c});
    t.pixels.push(Pixel{x: 5, y:-1, c: c});
    t.pixels.push(Pixel{x: 5, y:-2, c: c}); // upper
}

fn fill_new_t(t: &mut Tetronimo) {
    let c = Color::Magenta;

    t.pixels.push(Pixel{x: 4, y:-2, c: c});
    t.pixels.push(Pixel{x: 4, y:-1, c: c});
    t.pixels.push(Pixel{x: 3, y:-1, c: c});
    t.pixels.push(Pixel{x: 5, y:-1, c: c});
}

fn fill_new_s(t: &mut Tetronimo) {
    let c = Color::Green;

    t.pixels.push(Pixel{x: 3, y:-1, c: c});
    t.pixels.push(Pixel{x: 4, y:-1, c: c});
    t.pixels.push(Pixel{x: 4, y:-2, c: c});
    t.pixels.push(Pixel{x: 5, y:-2, c: c});
}

fn fill_new_o(t: &mut Tetronimo) {
    let c = Color::LightYellow;

    t.pixels.push(Pixel{x: 4, y:-2, c: c});
    t.pixels.push(Pixel{x: 5, y:-2, c: c});
    t.pixels.push(Pixel{x: 4, y:-1, c: c});
    t.pixels.push(Pixel{x: 5, y:-1, c: c});
}
