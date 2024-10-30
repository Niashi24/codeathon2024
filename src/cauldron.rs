use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Ingredient {
    worry: u64,
    magic: u64,
}

fn read_line() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}

pub fn main() {
    let n: usize = read_line().parse().unwrap();

    let ingredients: Vec<Ingredient> = (0..n)
        .map(|_| {
            let line = read_line();
            let (w, v) = line.split_once(" ").unwrap();
            Ingredient {
                worry: w.parse().unwrap(),
                magic: v.parse().unwrap(),
            }
        })
        .collect();

    let p: usize = read_line().parse().unwrap();
    let mut memo = HashMap::new();

    for _ in 0..p {
        let line = read_line();
        let (max, x) = line.split_once(" ").unwrap();
        let max: u64 = max.parse().unwrap();
        let x: usize = x.parse().unwrap();

        let answer = knapsack_rec(max, x - 1, &ingredients, &mut memo);
        println!("{answer}");
    }
}

fn knapsack_rec(
    limit: u64,
    i: usize,
    ings: &[Ingredient],
    memo: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if let Some(&mem) = memo.get(&(limit, i)) {
        return mem;
    }

    let out = if ings[i].worry > limit {
        if i == 0 {
            return 0;
        }
        knapsack_rec(limit, i - 1, ings, memo)
    } else {
        if i == 0 {
            ings[i].magic
        } else {
            u64::max(
                ings[i].magic + knapsack_rec(limit - ings[i].worry, i - 1, ings, memo),
                knapsack_rec(limit, i - 1, ings, memo),
            )
        }
    };

    memo.insert((limit, i), out);
    out
}
