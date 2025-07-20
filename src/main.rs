use std::mem::swap;

const WIDTH: i32 = 10;
const STEP: f32 = 0.3;
const OFFSET: i32 = 12;
const LENGTH: i32 = 40;

const BASE: &str = "=";
// const BASE: &str = "-";
const SPIRAL: &str = "@@";

fn main() {
    for i in 0..LENGTH {
        let mut left = (WIDTH as f32 * ((STEP * i as f32).sin() + 1 as f32)) as usize;
        let mut right = (WIDTH as f32 * ((STEP * (i + OFFSET) as f32).sin() + 1 as f32)) as usize;
        if left > right {
            swap(&mut left, &mut right);
        }
        println!("{}{SPIRAL}{}{SPIRAL}", " ".repeat(left), BASE.repeat(right - left));
    }
}