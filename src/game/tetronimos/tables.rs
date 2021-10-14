use super::{Type, Pixel};

use tui::style::Color;

// we use tables for tetronimos spawn
// as the easiest way
pub fn fill_new_pixels(t: Type) -> [Pixel; 4] {
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
