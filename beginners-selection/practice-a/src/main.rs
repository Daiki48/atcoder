use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();

    let mut bc = String::new();
    io::stdin().read_line(&mut bc).unwrap();
    let mut bc = bc.split_whitespace();
    let b: i32 = bc.next().unwrap().parse().unwrap();
    let c: i32 = bc.next().unwrap().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    if 0 < a && a < 1000 && 0 < b && b < 1000 && 0 < c && c < 1000 && s.len() < 100 {
        let sum = a + b + c;
        println!("{} {}", sum, s);
    }
}
