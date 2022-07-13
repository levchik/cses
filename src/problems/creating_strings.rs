use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{stdin, stdout};

fn swap(a: &Vec<char>, i: usize, j: usize) -> Vec<char> {
    let mut a = a.clone();
    let tmp = a[i];
    a[i] = a[j];
    a[j] = tmp;
    a
}

fn perm(new: &mut Vec<String>, a: &Vec<char>, start: usize) {
    for i in start..a.len() {
        for j in i..a.len() {
            if i != j {
                let new_arr = swap(a, i, j);
                let new_arr_str = new_arr.clone().into_iter().collect();
                new.push(new_arr_str);
                perm(new,&new_arr, i + 1);
            }
        }
    }
}

pub fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).expect("Failed read");
    let s: String = n_input.trim_end().parse().unwrap();
    let char_vec: Vec<char> = s.chars().collect();
    let mut new: Vec<String> = Vec::new();
    perm(&mut new, &char_vec, 0);
    new.push(s);

    let new_set: HashSet<String> = new.iter().cloned().collect();
    let mut result_vec: Vec<String> = new_set.into_iter().collect::<Vec<String>>();

    result_vec.sort();
    println!("{}", result_vec.len());
    for i in result_vec {
        println!("{}", i);
    }
}