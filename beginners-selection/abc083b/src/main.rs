use std::io;

fn sum_digits(mut num: i32) -> i32 {
    let mut sum: i32 = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

fn main() {
    let mut nab = String::new();
    io::stdin().read_line(&mut nab).unwrap();
    let mut nab = nab.split_whitespace();
    let n: i32 = nab.next().unwrap().trim().parse().unwrap();
    let a: i32 = nab.next().unwrap().trim().parse().unwrap();
    let b: i32 = nab.next().unwrap().trim().parse().unwrap();

    let mut ans: i32 = 0;
    if (1..=10i32.pow(4)).contains(&n) && (1..b).contains(&a) && (a + 1..=36).contains(&b) {
        for i in 1..=n {
            let result = sum_digits(i);
            // println!("{} : {}", i, &result);
            if (a..=b).contains(&result) {
                ans += i;
            }
        }
    } else {
        println!("Invalid input");
    }
    println!("{}", ans);
}
