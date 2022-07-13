#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let mut n: i64 = n_input.trim_end().parse().unwrap();

    let mut sum: i64 = 0;
    loop {
        n = n / 5;
        sum += n;
        if n < 5 {break};
    }

    println!("{}", sum);
}