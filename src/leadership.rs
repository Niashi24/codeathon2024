use std::collections::HashMap;
use std::convert::TryInto;

pub fn main() {
    let t: usize = read_line().parse().unwrap();

    for _ in 0..t {
        let (n, _a) = read_line()
            .split_once(" ")
            .unwrap();
        
        let n: i32 = n.parse().unwrap();

        let line = read_line();
        let name_to_idx: HashMap<&str, usize> = line
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();

        let mut graph: HashMap<usize, Node> = HashMap::new();

        // initialize graph with those that need approval
        for _ in 0..n {
            let line = read_line();
            let mut tokens = line.split_whitespace();
            let name = tokens.next().unwrap().to_string();
            let need: usize = tokens.next().unwrap().parse().unwrap();
            let _ = tokens.next();

            let x = *name_to_idx.get(name.as_str()).unwrap();
            // get the node with that index, creating it if need be
            let mut node = graph.entry(x).or_default().clone();
            // node that for nodes 
            node.remaining = need;

            for i in read_line()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
            {
                graph.entry(i).or_default().needed.push(x);
            }

            graph.insert(x, node);
        }

        // start by processing nodes without 
        let mut to_process: Vec<usize> = graph
            .iter()
            .filter(|(_, req)| req.remaining == 0)
            .map(|(name, _)| *name)
            .collect();

        let mut total = name_to_idx.len() - graph.len();

        while let Some(next) = to_process.pop() {
            total += 1;
            let node = graph.remove(&next).unwrap();
            for i in node.needed {
                if let Some(req) = graph.get_mut(&i) {
                    match req.remaining {
                        0 => {}  // either already processed or to be processed
                        1 => {
                            req.remaining = 0;
                            to_process.push(i);
                        }
                        _ => {
                            req.remaining -= 1;
                        }
                    }
                }
            }
        }

        println!("{total}");
    }
}

#[derive(Default, Clone)]
struct Node {
    remaining: usize,
    needed: Vec<usize>,
}

fn read_line() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out.trim().to_string()
}
