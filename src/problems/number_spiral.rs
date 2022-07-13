#[allow(unused_imports)]
use std::io::{stdin, stdout};

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let n: i128 = n_input.trim_end().parse().unwrap();

    let mut answers: Vec<String> = Vec::new();
    for i in 1..n+1 {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        let nums: Vec<i128> = input.trim_end()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        // So, calculating the whole spiral, then just getting element is obviously very slow

        // So, odd row is going left->right bottom->top, even row is going right->left, top->bottom

        // For example if row is 4 and column is 2, we know that we're in 4x4 square,
        // This means we have largest number 16, and since it is even row, 16 is on the left, bottom
        // So, basically, 16 - 1 (2nd column) = 15.

        // If other way around, row is 2, column is 4, we still, know we're in 4x4 square,
        // And we know that we if shift by 4 to the right and 2 to the top we get result
        // So, basically, 16 - 3 = 13, 13 - 2 (since it is rightmost number) = 11

        // Let's see odd numbers example
        // row = 3, column = 5, we're in 5x5 square, since column is even, 25 is at the top right
        // so, moving it two rows down, gives us the answer - 23

        // y is ROW, x is COLUMN
        let y = nums[0];
        let x = nums[1];
        let square_side = std::cmp::max(y, x);
        let max_number = square_side * square_side;
        if square_side % 2 == 0 {
            answers.push(format!("{}", max_number - (x - 1) - (square_side - y)));
        } else {
            answers.push(format!("{}", max_number - (y - 1) - (square_side - x)));
        }
    }
    for answer in answers.iter() {
        println!("{}", answer)
    }
}