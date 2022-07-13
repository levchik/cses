#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i128 = n_input.trim_end().parse().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed read");
    let mut nums: Vec<i128> = input.trim_end()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut moves: i128 = 0;
    let mut prev: i128 = 0;

    for i in 0..n {
        let i = i as usize;
        if i == 0 {
            prev = nums[i];
            continue
        }

        if nums[i] < prev {
            moves += prev - nums[i];
        } else {
            prev = nums[i];
        }
    }
    println!("{}", moves);

}
