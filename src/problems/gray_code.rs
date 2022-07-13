#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: usize = n_input.trim_end().parse().unwrap();
    for i in 0..2_i32.pow(n as u32) {
        println!("{:0n$b}", i^(i>>1), n=n);
    }
}
