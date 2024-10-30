use std::collections::HashSet;
use std::io::stdin;

pub fn main() {
    let _n: usize = read_line().parse().unwrap();
    let words = read_line();
    let (max_str, max_score) = words.split_whitespace()
        .map(|s| (s, s.chars().collect::<HashSet<_>>().len()))
        .max_by_key(|s| s.1)
        .unwrap();

    println!("{} {}", max_str, max_score);
}

fn read_line() -> String {
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
