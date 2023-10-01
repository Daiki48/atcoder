use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort_by(|a, b| b.cmp(a));

    if n > 1 {
        let a_point: i32 = sorted_nums.iter().step_by(2).sum();
        let b_point: i32 = sorted_nums.iter().skip(1).step_by(2).sum();
        println!("{}", a_point - b_point);
    } else {
        println!("{}", sorted_nums[0]);
    }
}
