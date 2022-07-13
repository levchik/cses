#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: usize = n_input.trim_end().parse().unwrap();
    let moves = 2_i32.pow(n as u32) - 1;
    println!("{}", moves);
    for m in 1..moves + 1 {
        // move m is from peg (m & m - 1) % 3 to peg ((m | m - 1) + 1) % 3
        let mut start_peg = (m - (m & -m)) % 3 + 1;
        let mut end_peg = (m + (m & -m)) % 3 + 1;
        if n % 2 == 0 {
            if start_peg == 2 {
                start_peg = 3;
            } else if start_peg == 3 {
                start_peg = 2;
            }

            if end_peg == 2 {
                end_peg = 3;
            } else if end_peg == 3 {
                end_peg = 2;
            }
        }
        println!("{} {}", start_peg, end_peg);
    }
}
