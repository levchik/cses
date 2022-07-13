#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: u32 = n_input.trim_end().parse().unwrap();
    const M: u128 = 1000000007;
    let mut res: u128 = 2;
    for i in 1..n {
        res = (2 * res) % M;
    }
    println!("{}", res);
}