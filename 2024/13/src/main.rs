use core::f64;
use std::{env, fs};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Button {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: Position,
}

fn get_data() -> Vec<ClawMachine> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split("\n\n")
        .map(|m| {
            let re = Regex::new(r".*X.(\d+),.*Y.(\d+)").unwrap();
            let captures = re
                .captures_iter(m)
                .map(|cap| {
                    (
                        cap[1].parse::<f64>().unwrap(),
                        cap[2].parse::<f64>().unwrap(),
                    )
                })
                .collect::<Vec<(f64, f64)>>();
            ClawMachine {
                a: Button {
                    x: captures[0].0,
                    y: captures[0].1,
                },
                b: Button {
                    x: captures[1].0,
                    y: captures[1].1,
                },
                prize: Position {
                    x: captures[2].0,
                    y: captures[2].1,
                },
            }
        })
        .collect()
}

fn try_find_cheapest(machine: &ClawMachine, padding: f64) -> f64 {
    let mut m = machine.clone();
    m.prize.x += padding;
    m.prize.y += padding;
    let a = (m.prize.x * m.b.y - m.prize.y * m.b.x) / (m.a.x * m.b.y - m.a.y * m.b.x);
    let b = (m.prize.y * m.a.x - m.prize.x * m.a.y) / (m.a.x * m.b.y - m.a.y * m.b.x);
    if a == a.round() && b == b.round() {
        3.0 * a + b
    } else {
        0.0
    }
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!(
            "{}",
            input
                .iter()
                .fold(0.0, |cost, m| cost + try_find_cheapest(m, 10000000000000.0))
        ),
        _ => println!(
            "{}",
            input
                .iter()
                .fold(0.0, |cost, m| { cost + try_find_cheapest(m, 0.0) })
        ),
    }
}
