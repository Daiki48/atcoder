use std::io;

fn main() {
    let mut ab = String::new();
    io::stdin().read_line(&mut ab).unwrap();
    let mut ab = ab.split_whitespace();
    let a: i32 = ab.next().unwrap().parse().unwrap();
    let b: i32 = ab.next().unwrap().parse().unwrap();
    if (1..=10000).contains(&a) && (1..=10000).contains(&b) {
        // println!("{} {}", a, b);
        let result = a * b;
        if result % 2 == 0 {
            println!("Even");
        } else {
            println!("Odd");
        }
    } else {
        println!("Not range number");
    }
}
