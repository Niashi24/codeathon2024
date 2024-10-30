use std::collections::HashSet;
use std::io::stdin;

pub fn main() {
    let _n: usize = read_line().parse().unwrap();
    let words = read_line();
    let mut max_str = "";
    let mut max_score = 0;
    for word in words.split_whitespace() {
        let count = word.chars().collect::<HashSet<_>>().len();
        if count > max_score {
            max_str = word;
            max_score = count;
        }
    }

    println!("{} {}", max_str, max_score);
}

fn read_line() -> String {
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
