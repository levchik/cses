#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i128 = n_input.trim_end().parse().unwrap();
    for k in 1..n+1 {
        if k == 1 {
            println!("0")
        } else {
            println!("{}", (k*k*(k*k - 1) / 2) - 4 * (k-1) * (k-2))
        }
    }
}
