use std::collections::HashMap;

pub fn main() {
    let ops: HashMap<char, Operation> = read_line()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let op = match read_line().split_once(" ").unwrap().1 {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                x => panic!("unknown operator: {x}"),
            };
            (c, Operation { priority: i, op })
        })
        .collect();

    let n: usize = read_line().parse().unwrap();

    for _ in 0..n {
        let line = read_line();
        let mut tokens = line.split_whitespace();
        println!("{}", solve(&mut tokens, &ops));
    }
}

fn solve<'a>(line: &mut impl Iterator<Item = &'a str>, ops: &HashMap<char, Operation>) -> i64 {
    let mut nums = Vec::new();
    let mut operations = Vec::new();

    nums.push(line.next().unwrap().parse().unwrap());

    loop {
        match line.next() {
            None | Some(")") => return evaluate(nums, operations),
            Some(op) => {
                operations.push(*ops.get(&op.chars().next().unwrap()).unwrap());
                nums.push(match line.next().unwrap() {
                    "(" => solve(line, ops),
                    x => x.parse().unwrap(),
                });
            }
        }
    }
}

fn evaluate(mut nums: Vec<i64>, mut operators: Vec<Operation>) -> i64 {
    while !operators.is_empty() {
        let idx = operators
            .iter()
            .enumerate()
            .min_by_key(|x| x.1.priority)
            .unwrap()
            .0;
        let op = operators.remove(idx);
        let a = nums[idx];
        let b = nums.remove(idx + 1);
        nums[idx] = match op.op {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
        }
    }

    nums[0]
}

#[derive(Copy, Clone)]
struct Operation {
    priority: usize,
    op: Operator,
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
}

fn read_line() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
