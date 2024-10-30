use std::io::stdin;

fn main() {
    let _num_tiles: usize = read_line().parse().unwrap();
    let seconds: usize = read_line().parse().unwrap();
    let mut tiles: Vec<usize> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pos = 0;
    for _ in 0..seconds {
        pos += 1;

        match tiles.get(pos) {
            None => {
                pos = 0;
            }
            Some(0) => {
                tiles[pos] = 5;
                pos = 0;
            }
            _ => {}
        }
    }

    println!("{}", tiles[pos])
}

fn read_line() -> String {
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
