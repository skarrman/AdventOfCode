use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Hash, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn get_data() -> (HashSet<Point>, Vec<(String, usize)>) {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not read file");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let points = parts[0]
        .split("\n")
        .map(|ns| {
            let coords = ns.split(",").collect::<Vec<&str>>();
            Point {
                x: coords[0].parse::<usize>().unwrap(),
                y: coords[1].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let folds = parts[1]
        .split("\n")
        .map(|l| {
            let ps = l.split("=").collect::<Vec<&str>>();
            (
                String::from(if ps[0].contains("x") { "x" } else { "y" }),
                ps[1].parse::<usize>().unwrap(),
            )
        })
        .collect();
    (points, folds)
}

fn abs(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn max(x: usize, y: usize) -> usize {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let (mut points, folds) = get_data();
    for (axis, num) in folds {
        points = points
            .iter()
            .map(|p| match axis.as_str() {
                "x" => Point {
                    x: num - abs(p.x, num),
                    y: p.y,
                },
                "y" => Point {
                    x: p.x,
                    y: num - abs(p.y, num),
                },
                _ => panic!(),
            })
            .collect();
        if part == "part1" {
            println!("{}", points.len());
            return;
        }
    }
    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |(m_x, m_y), p| (max(m_x, p.x), max(m_y, p.y)));
    let mut sheet = vec![vec![" "; max_x + 1]; max_y + 1];
    for p in points {
        sheet[p.y][p.x] = "#";
    }
    for row in sheet {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
