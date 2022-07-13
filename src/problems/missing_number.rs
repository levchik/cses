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

    nums.sort();
    let mut prev: i128 = 0;
    for i in 0..n-1 {
        if nums[i as usize] == prev + 2 {
            println!("{}", prev + 1);
            return
        }
        prev = nums[i as usize];
    }
    println!("{}", n);

}
