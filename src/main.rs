use std::mem::swap;

const WIDTH: f64 = 10.0;
const STEP: f64 = 0.3;
const OFFSET: i32 = 12;
const LENGHT: i32 = 40;

const BASE: &str = "-"; 
const SPIRAL: &str = "@@";

fn main() {
    for i in 0..LENGHT {
        let mut left = (WIDTH * ((STEP * i as f64).sin() + 1 as f64)) as usize;
        let mut right = (WIDTH * ((STEP * (i + OFFSET) as f64).sin() + 1 as f64)) as usize;
        if left > right {
            swap(&mut left, &mut right);
        }
        println!("{}{SPIRAL}{}{SPIRAL}", " ".repeat(left), BASE.repeat(right - left));
    }
}