#[allow(unused_imports)]
use std::char;
use std::io::{stdin, stdout};

pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed read");

    let mut prev_char: char = char::from_u32(0x2764).unwrap();
    let mut curr_sum: i128 = 0;
    let mut curr_max: i128 = 0;
    for ch in input.chars() {
        if ch == prev_char {
            curr_sum += 1;
        } else {
            curr_max = std::cmp::max(curr_max, curr_sum);
            curr_sum = 1;
        }
        prev_char = ch;
    }
    println!("{}", curr_max);
}
