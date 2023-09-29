use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b: i32 = b.trim().parse().unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let c: i32 = c.trim().parse().unwrap();

    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: i32 = x.trim().parse().unwrap();

    let abc = a + b + c;
    let mut result: i32 = 0;

    if (0..=50).contains(&a)
        && (0..=50).contains(&b)
        && (0..=50).contains(&c)
        && 1 <= abc
        && (50..=20000).contains(&x)
        && &x % 50 == 0
    {
        for i in 0..a + 1 {
            for j in 0..b + 1 {
                for k in 0..c + 1 {
                    let z = 500 * i + 100 * j + 50 * k;
                    if z == x {
                        result += 1;
                    }
                }
            }
        }
        println!("{}", result);
    } else {
        println!("Invalid input");
    }
}
