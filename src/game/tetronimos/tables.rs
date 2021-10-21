use super::{Pixel, Type};

use tui::style::Color;

// we use tables for tetronimos spawn
// as the easiest way
pub fn fill_new_pixels(t: Type) -> [Pixel; 4] {
    let res = [Pixel {
        x: 0,
        y: 0,
        c: Color::White,
    }; 4];
    match t {
        Type::I => fill_new_i(res),
        Type::L => fill_new_l(res),
        Type::T => fill_new_t(res),
        Type::O => fill_new_o(res),
        Type::S => fill_new_s(res),
        Type::J => fill_new_j(res),
        Type::Z => fill_new_z(res),
    }
}

fn fill_new_i(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Cyan;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 3, y: -1, c: c };
    ps[2] = Pixel { x: 5, y: -1, c: c };
    ps[3] = Pixel { x: 6, y: -1, c: c };

    ps
}

fn fill_new_l(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Yellow;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 3, y: -1, c: c };
    ps[2] = Pixel { x: 5, y: -1, c: c };
    ps[3] = Pixel { x: 5, y: -2, c: c };

    ps
}

fn fill_new_t(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Magenta;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 4, y: -2, c: c };
    ps[2] = Pixel { x: 3, y: -1, c: c };
    ps[3] = Pixel { x: 5, y: -1, c: c };

    ps
}

fn fill_new_s(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Green;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 3, y: -1, c: c };
    ps[2] = Pixel { x: 4, y: -2, c: c };
    ps[3] = Pixel { x: 5, y: -2, c: c };

    ps
}

fn fill_new_o(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::LightYellow;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 4, y: -2, c: c };
    ps[2] = Pixel { x: 5, y: -2, c: c };
    ps[3] = Pixel { x: 5, y: -1, c: c };

    ps
}

fn fill_new_j(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::LightBlue;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 3, y: -1, c: c };
    ps[2] = Pixel { x: 3, y: -2, c: c };
    ps[3] = Pixel { x: 5, y: -1, c: c };

    ps
}

fn fill_new_z(mut ps: [Pixel; 4]) -> [Pixel; 4] {
    let c = Color::Red;

    ps[0] = Pixel { x: 4, y: -1, c: c };
    ps[1] = Pixel { x: 3, y: -2, c: c };
    ps[2] = Pixel { x: 4, y: -2, c: c };
    ps[3] = Pixel { x: 5, y: -1, c: c };

    ps
}

const OFFSETS_O: [[(i8, i8); 1]; 4] = [[(0, 0)], [(0, -1)], [(-1, -1)], [(-1, 0)]];
const OFFSETS_I: [[(i8, i8); 5]; 4] = [
    [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
    [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
    [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
];
const OFFSETS_COMMON: [[(i8, i8); 5]; 4] = [
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
];

pub fn get_kick_offsets(from: i8, to: i8, t: Type) -> Vec<(i8, i8)> {
    match t {
        Type::O => get_kicks_o(from, to),
        Type::I => get_kicks_i(from, to),
        _ => get_kicks_common(from, to),
    }
}

fn get_kicks_o(from: i8, to: i8) -> Vec<(i8, i8)> {
    let o_f = OFFSETS_O[from as usize][0];
    let o_t = OFFSETS_O[to as usize][0];

    vec![(o_f.0 - o_t.0, o_f.1 - o_t.1)]
}

fn get_kicks_i(from: i8, to: i8) -> Vec<(i8, i8)> {
    let mut res = Vec::new();
    for i in 0..5 {
        let o_f = OFFSETS_I[from as usize][i];
        let o_t = OFFSETS_I[to as usize][i];

        res.push((o_f.0 - o_t.0, o_f.1 - o_t.1));
    }
    res
}

fn get_kicks_common(from: i8, to: i8) -> Vec<(i8, i8)> {
    let mut res = Vec::new();
    for i in 0..5 {
        let o_f = OFFSETS_COMMON[from as usize][i];
        let o_t = OFFSETS_COMMON[to as usize][i];

        res.push((o_f.0 - o_t.0, o_f.1 - o_t.1));
    }
    res
}
