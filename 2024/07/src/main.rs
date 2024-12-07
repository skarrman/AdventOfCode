use std::{env, fs};

fn get_data() -> Vec<(u128, Vec<u128>)> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split("\n")
        .map(|row| {
            let split = row.split(": ").collect::<Vec<&str>>();
            (
                split[0].parse::<u128>().unwrap(),
                split[1]
                    .split(" ")
                    .map(|d| d.parse::<u128>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn generate_combinations(ops: &Vec<Op>, vars: usize) -> Vec<Vec<Op>> {
    let mut comb: Vec<Vec<Op>> = ops.iter().map(|op| vec![*op]).collect();
    for _ in 0..(vars - 1) {
        let mut res: Vec<Vec<Op>> = Vec::new();
        for i in 0..comb.len() {
            for op in ops {
                let mut temp = comb[i].clone();
                temp.push(*op);
                res.push(temp);
            }
        }
        comb = res;
    }
    comb
}

fn solve(test_value: u128, operators: &Vec<u128>, valid_ops: &Vec<Op>) -> u128 {
    let combs = generate_combinations(&valid_ops, operators.len() - 1);
    for comb in combs {
        let mut res = operators[0];
        for i in 0..operators.len() - 1 {
            res = match comb[i] {
                Op::Add => res + operators[i + 1],
                Op::Mul => res * operators[i + 1],
                Op::Con => format!("{}{}", res, operators[i + 1])
                    .parse::<u128>()
                    .unwrap(),
            }
        }
        if res == test_value {
            return test_value;
        }
    }
    0
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Con,
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => {
            println!(
                "{}",
                input.iter().fold(0, |sum, (t, o)| sum
                    + solve(*t, o, &vec![Op::Add, Op::Mul, Op::Con]))
            )
        }
        _ => println!(
            "{}",
            input
                .iter()
                .fold(0, |sum, (t, o)| sum + solve(*t, o, &vec![Op::Add, Op::Mul]))
        ),
    }
}
