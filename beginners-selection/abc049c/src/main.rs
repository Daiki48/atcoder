use std::io::{self, BufRead};

fn main() {
    let mut line = io::stdin().lock().lines();
    let s: String = line.next().unwrap().unwrap().parse().unwrap();

    let s = s.chars().rev().collect::<String>();
    let mut t = String::new();
    let words = ["maerd", "remaerd", "esare", "resare"];

    let mut i = 0;
    while i < s.len() {
        let mut found = false;
        for &word in &words {
            if s[i..].starts_with(word) {
                t.push_str(word);
                i += word.len();
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
    if t == s {
        println!("YES");
    } else {
        println!("NO");
    }
}
