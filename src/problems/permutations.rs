#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed read");
    let n: i128 = input.trim_end().parse().unwrap();
    // Well, computing all permutations and checking each one of them until we find answer is slow
    // n < 3, -> No solution
    // n = 4, -> 3, 1, 4, 2 -> odd numbers from max to min, then even numbers from max to min
    // n = 5, -> 5, 3, 1, 4, 2
    // n = 6, -> 5, 3, 1, 6, 4, 2
    // Now, we see, we have two lists of odd and even numbers, those lists are populated
    // with pushing new elements from the left side, and then concatenated.
    if n == 1 {
        print!("1");
        return
    } else if n <= 3 {
        print!("NO SOLUTION");
        return
    }

    let mut odd_nums: Vec<i128> = Vec::new();
    for i in 1..n + 1 {
        if i % 2 == 0 {
            print!("{} ", i);
        } else {
            odd_nums.push(i);
        }
    }
    odd_nums.iter().fold(true, |first, elem| {
        if !first { print!(" "); }
        print!("{}", elem);
        false
    });
}