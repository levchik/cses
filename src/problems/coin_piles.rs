#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i64 = n_input.trim_end().parse().unwrap();

    let mut answers: Vec<&str> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        let nums: Vec<i128> = input.trim_end()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let a = nums[0];
        let b = nums[1];
        let multiplier: f64;
        if a > b {
            multiplier = a as f64 / b as f64;
        } else {
            multiplier = b as f64 / a as f64;
        }

        if a == 0 && b == 0 {
            answers.push("YES")
        } else if (a+b) % 3 == 0 && multiplier <= 2.0 {
            answers.push("YES")
        } else {
            answers.push("NO")
        }

    }
    for ans in answers.iter() {
        println!("{}", ans);
    }
}