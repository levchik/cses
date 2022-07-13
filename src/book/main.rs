use std::cmp::max;


fn largest_sum_subarray(vec: &Vec<i32>) -> &[i32] {
    let mut sum: i32 = 0;
    let mut best: i32 = 0;
    let mut best_start: usize = 0;
    let mut best_end: usize = 0;
    for (index, item) in vec.iter().enumerate() {
        let prev_sum = sum;
        if prev_sum < 0 {
            best_start = index;
        }
        sum = max(*item, sum + *item);
        if sum >= best {
            best = sum;
            best_end = index;
        }
    }
    return &vec[best_start..best_end+1];
}

fn main() {
    let one = vec![-1, 2, 4, -3, 5, 2, -5, 2];
    let two = vec![-10, 2, 4, -3, 5, 2, -5, 2];
    let three = vec![10, 2, 4, -3, 5, 2, -5, 2];
    let four = vec![-1, 2, 4, -100, 5, 2, -5, 2];
    let five = vec![-1, 2, 4, -3, 5, 2, 1, -2, 6, 7];
    let six = vec![-1, 2, 4, -3, 5, 2, 1, 2];

    let one_subarr = largest_sum_subarray(&one);
    let two_subarr = largest_sum_subarray(&two);
    let three_subarr = largest_sum_subarray(&three);
    let four_subarr = largest_sum_subarray(&four);
    let five_subarr = largest_sum_subarray(&five);
    let six_subarr = largest_sum_subarray(&six);

    println!("For one, SUM={:#?} best summed subarray is {:?}", one_subarr.iter().sum::<i32>(), one_subarr);
    println!("For two, SUM={:#?} best summed subarray is {:?}", two_subarr.iter().sum::<i32>(), two_subarr);
    println!("For three, SUM={:#?} best summed subarray is {:?}", three_subarr.iter().sum::<i32>(), three_subarr);
    println!("For four, SUM={:#?} best summed subarray is {:?}", four_subarr.iter().sum::<i32>(), four_subarr);
    println!("For five, SUM={:#?} best summed subarray is {:?}", five_subarr.iter().sum::<i32>(), five_subarr);
    println!("For six, SUM={:#?} best summed subarray is {:?}", six_subarr.iter().sum::<i32>(), six_subarr);
}
