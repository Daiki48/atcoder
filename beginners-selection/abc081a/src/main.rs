use std::io;

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let chars: Vec<char> = s.chars().collect();
    // let a = chars[0].to_digit(10).unwrap();
    // let b = chars[1].to_digit(10).unwrap();
    // let c = chars[2].to_digit(10).unwrap();

    let mut count = 0;

    for i in &chars {
        if *i == '1' {
            count += 1;
        }
    }

    // println!("{} {} {}", a, b, c);
    println!("{}", count);

    Ok(())

}
