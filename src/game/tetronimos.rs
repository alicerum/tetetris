use tui::style::Color;
use super::Pixel;

pub enum Type {
    I,
    L,
    T,
}


pub struct Tetronimo {
    t: Type,

    pixels: Vec<Pixel>,
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
        }

        t
    }

    pub fn move_tick(&mut self) {
        let mut at_bottom = false;
        for p in &self.pixels {
            if p.y == 19 {
                at_bottom = true;
                break;
            }
        }

        if !at_bottom {
            self.pixels.iter_mut().for_each(|p| {
                p.y += 1;
            });
        }
    }

    pub fn check_pixel(&self, x: usize, y: usize) -> Option<Color> {
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
}

fn fill_new_l(t: &mut Tetronimo) {
    let c = Color::Yellow;
}

fn fill_new_t(t: &mut Tetronimo) {
    let c = Color::Red;

    t.pixels.push(Pixel{x: 4, y:0, c: c});
    t.pixels.push(Pixel{x: 4, y:1, c: c});
    t.pixels.push(Pixel{x: 3, y:1, c: c});
    t.pixels.push(Pixel{x: 5, y:1, c: c});
}
