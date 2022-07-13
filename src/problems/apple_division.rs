#[allow(unused_imports)]
use std::io::{stdin, stdout};


pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i64 = n_input.trim_end().parse().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed read");
    let nums: Vec<i128> = input.trim_end()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let total: i128 = nums.iter().sum();
    let mut ans: i128 = (1 << 63);

    // i -> from 0 to 2^n
    // j -> from 0 to n
    // if i & 2^j
    for i in 0..1 << n {
        let mut s = 0;
        for j in 0..n {
            let d = (1 << j);
            let dand = (i & d);
            if dand != 0 {
                s += nums[j as usize];
            }
        }
        let curr = ((total - s) - s).abs();
        ans = ans.min(curr);
    }
    println!("{}", ans)
}