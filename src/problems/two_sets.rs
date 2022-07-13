#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i128 = n_input.trim_end().parse().unwrap();

    if n <= 2 {
        println!("NO");
        return
    }

    let nsum: i128 = (1..n+1).sum();
    if nsum % 2 != 0 {
        println!("NO");
        return
    } else {
        println!("YES");
    }

    let target_subset_sum: i128 = nsum / 2;
    let mut inter_sum: i128 = 0;
    let mut subset_one: Vec<i128> = Vec::new();
    let mut subset_two: Vec<i128> = Vec::new();
    for i in (1..n+1).rev() {
        if inter_sum < target_subset_sum {
            if inter_sum + i <= target_subset_sum {
                subset_one.push(i);
                inter_sum += i;
            } else {
                subset_two.push(i);
            }
        } else {
            subset_two.push(i);
        }
    }
    println!("{}", subset_one.len());
    subset_one.iter().fold(true, |first, elem| {
        if !first { print!(" "); }
        print!("{}", elem);
        false
    });
    println!();
    println!("{}", subset_two.len());
    subset_two.iter().fold(true, |first, elem| {
        if !first { print!(" "); }
        print!("{}", elem);
        false
    });
}
