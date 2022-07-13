#[allow(unused_imports)]
use std::io::{stdin, stdout};


pub fn order(k: i128, mut adder: i128, mut divider: i128) -> (i128, i128) {
    let mut total_adder: i128 = adder;
    let mut i: i128 = (k + total_adder) / divider;
    while i / (adder * 10) > 0 {
        adder *= 10;
        total_adder += adder;
        divider += 1;
        i = (k + total_adder) / divider;
    }
    return (total_adder, divider)
}

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: usize = n_input.trim_end().parse().unwrap();
    let mut ans: Vec<String> = Vec::new();
    for i in 0..n {
        let mut k_input = String::new();
        stdin().read_line(&mut k_input).expect("Failed read");
        let k: i128 = k_input.trim_end().parse().unwrap();
        if k < 10 {
            ans.push(format!("{}", k));
        } else {
            let result = order(k, 10, 2);
            let adder = result.0;
            let divider = result.1;
            let index = ((k + adder) % divider) as usize;
            let c: Vec<char> = ((k + adder) / divider).to_string().chars().collect();
            ans.push(format!("{}", c[index]));
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
