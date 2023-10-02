use std::io::{self, BufRead};
fn main() {
    let mut lines = io::stdin().lock().lines();
    let n: isize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = Vec::new();

    if n > 1 {
        for _ in 0..n {
            let num: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            nums.push(num);
        }
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_by(|a, b| b.cmp(a));
        sorted_nums.dedup();
        // println!("sorted_nums : {:?}", sorted_nums);
        println!("{}", sorted_nums.len());
    } else if n == 1 {
        let num: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        nums.push(num);
        println!("{}", nums[0]);
    } else {
        println!("Invalid input");
    }
    // println!("{}", n);
    // println!("nums : {:?}", nums);
}
