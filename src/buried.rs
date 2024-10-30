pub fn main() {
    let n: i64 = read_line().parse().unwrap();
    for _ in 0..n {
        let line = read_line();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        let x = nums.next().unwrap();

        println!("{}", solution(a, b, x));
    }
}

fn solution(a: i64, b: i64, x: i64) -> i64 {
    if !can_solve(a, b, x) {
        return -1;
    }

    let mut i = 0;
    loop {
        for n in 0..=i {
            if a * n - b * (i - n) == x {
                return i + 1;
            }
        }

        i += 1;
    }
}

pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m %= n;
    }
    n
}

fn can_solve(a: i64, b: i64, x: i64) -> bool {
    x % gcd(a, b) == 0
}

fn read_line() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
