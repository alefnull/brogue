#![allow(unused)]
use std::time::{SystemTime, UNIX_EPOCH};

use bracket_lib::{
    color::{CYAN, LIMEGREEN, MAGENTA, RED, RGB, WHITE},
    noise::FastNoise,
    terminal::{BTerm, Point},
};
use rand::{seq::IteratorRandom, Rng};

use crate::entities::entity::EntityFuncs;

// MARK: get_random_char()
pub fn get_random_char() -> char {
    let chars = "@#%".chars();
    let mut rng = rand::thread_rng();
    chars.choose(&mut rng).unwrap_or_default()
}

// MARK: get_random_color()
pub fn get_random_color() -> RGB {
    let cols = [
        RGB::named(RED),
        RGB::named(LIMEGREEN),
        RGB::named(MAGENTA),
        RGB::named(CYAN),
        RGB::named(WHITE),
    ];
    let mut rng = rand::thread_rng();
    cols.into_iter()
        .choose(&mut rng)
        .unwrap_or(RGB::named(WHITE))
}

// MARK: get_random_pos()
pub fn get_random_pos(max_w: i32, max_h: i32) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..max_w), rng.gen_range(0..max_h))
}

// MARK: fill_bg()
pub fn fill_bg(ctx: &mut BTerm, w: i32, h: i32, fg_color: RGB, bg_color: RGB, char: char) {
    ctx.cls();

    for x in 0..w {
        for y in 0..h {
            ctx.print_color(x, y, fg_color, bg_color, char);
        }
    }
}

// MARK: lerp()
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    a * (1.0 - t) + b * t
}

// MARK: lerp_exp()
pub fn lerp_exp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * t * t * (t * (6.0 * t - 15.0) + 10.0);
    a * (1.0 - t) + b * t
}

// MARK: lerp_color()
pub fn lerp_color(a: RGB, b: RGB, t: f32, exp: bool) -> RGB {
    if exp {
        let red = lerp_exp(a.r, b.r, t);
        let green = lerp_exp(a.g, b.g, t);
        let blue = lerp_exp(a.b, b.b, t);
        RGB::from_f32(red, green, blue)
    } else {
        let red = lerp(a.r, b.r, t);
        let green = lerp(a.g, b.g, t);
        let blue = lerp(a.b, b.b, t);
        RGB::from_f32(red, green, blue)
    }
}

// MARK: constrain_coords()
pub fn constrain_coords(x: i32, y: i32, w: i32, h: i32) -> Point {
    Point::new(x.max(0).min(w - 1), y.max(0).min(h - 1))
}

pub fn map_noise(mut seed: u64) -> FastNoise {
    if seed == 0 {
        seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    let mut noise = bracket_lib::noise::FastNoise::new();
    noise.set_seed(seed);
    noise.set_noise_type(bracket_lib::noise::NoiseType::ValueFractal);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);
    noise.set_fractal_octaves(4);
    noise
}
