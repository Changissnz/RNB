
use rand::prelude::*;

/*
std functions for prng
*/


pub fn random_f32_in_range(r:(f32,f32)) -> f32 {
    let q: f32 = random();
    r.0 + (r.1 - r.0) * q
}

pub fn random_i32_in_range(r:(i32,i32)) -> i32 {
    let q: f32 = random();
    (r.0 as f32 + q * (r.1 - r.0) as f32).round() as i32
}

pub fn random_char_in_range() -> char {
    let i = random_i32_in_range((0,52));
    let mut x:i32 = 65 + i;
    if x > 90 {
        x += 7;
    }
    char::from_u32(x as u32).unwrap()
}
