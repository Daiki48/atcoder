use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut nums = line.split_whitespace();
    let n: i32 = nums.next().unwrap().parse().unwrap();
    let money: i32 = nums.next().unwrap().parse().unwrap();

    let money_limit = 2 * 10i32.pow(7);
    if (1..=2000).contains(&n) && (1000..=money_limit).contains(&money) {
        for x in 0..=n {
            for y in 0..=(n - x) {
                let z = n - x - y;
                if money == 10000 * x + 5000 * y + 1000 * z {
                    println!("{} {} {}", x, y, z);
                    return;
                }
            }
        }
        println!("-1 -1 -1");
    } else {
        println!("Invalid input");
    }
}
