use std::io;

fn main() -> io::Result<()> {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let nn: Result<i32, _> = n.trim().parse();

    let mut abc = String::new();
    io::stdin().read_line(&mut abc).unwrap();
    let mut abc = abc.split_whitespace();
    let mut a: i32 = abc.next().unwrap().parse().unwrap();
    let mut b: i32 = abc.next().unwrap().parse().unwrap();
    let mut c: i32 = abc.next().unwrap().parse().unwrap();
    // println!("a:{}, b:{}, c:{}", &a, &b, &c);

    match nn {
        Ok(val) => {
            // nは1 <= n <= 200
            // if 1 <= val && val <= 200 {
            if (1..=200).contains(&val) {
                let mut count: i32 = 0;
                while &a % 2 == 0 && &b % 2 == 0 && &c % 2 == 0 {
                    a /= 2;
                    b /= 2;
                    c /= 2;
                    count += 1;
                }
                println!("{}", &count);
                // println!("val : {}", val);
            } else {
                println!("Over range : {}", val);
            }
        }
        Err(_e) => {
            println!("Invalid input");
        }
    }
    Ok(())
}
