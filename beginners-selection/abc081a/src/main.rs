use std::io;

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let chars: Vec<char> = s.chars().collect();

    let mut count = 0;

    for i in &chars {
        if *i == '1' {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}
