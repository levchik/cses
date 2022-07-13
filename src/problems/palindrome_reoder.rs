#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let s: String = n_input.trim_end().parse().unwrap();

    // Solution is O(2n)
    let char_vec: Vec<char> = s.chars().collect();
    let mut occurences: Vec<i128> = vec![0; 26];
    for c in char_vec {
        occurences[(c as usize - 65)] += 1;
    }

    let mut result = String::new();
    let mut odd_check: i128 = 0;
    for i in 0..26 {
        odd_check += (occurences[i] % 2);
        if odd_check >= 2 {
            println!("NO SOLUTION");
            return
        }
        if (occurences[i] % 2 == 0) {
            for _ in 0..occurences[i]/2 {result.push((i as u8 +65) as char)}
        }
    }
    for i in 0..26 {
        if !(occurences[i] % 2 == 0) {
            for _ in 0..occurences[i] {result.push((i as u8 +65) as char)}
        }
    }
    for i in (0..26).rev() {
        if (occurences[i] % 2 == 0) {
            for _ in 0..occurences[i]/2 {result.push((i as u8 +65) as char)}
        }
    }

    println!("{}", result);
}
