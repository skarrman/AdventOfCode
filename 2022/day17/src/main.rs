use std::collections::HashSet;
use std::env;
use std::fs;

const WIDTH: i64 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn get_data() -> Vec<Point> {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
        .chars()
        .map(|c| {
            if c == '<' {
                Point { x: -1, y: 0 }
            } else {
                Point { x: 1, y: 0 }
            }
        })
        .collect()
}

fn add(rock: &mut Vec<Point>, with: Point) {
    for p in rock {
        p.x += with.x;
        p.y += with.y;
    }
}

fn can_move(rock: &Vec<Point>, other_rocks: &HashSet<Point>, dir: Point) -> bool {
    for p in rock {
        if p.x + dir.x < 0
            || p.x + dir.x >= WIDTH
            || p.y + dir.y < 0
            || other_rocks.contains(&Point {
                x: p.x + dir.x,
                y: p.y + dir.y,
            })
        {
            return false;
        }
    }
    true
}

fn on_top_of_rock(rock: &Vec<Point>, other_rocks: &HashSet<Point>) -> bool {
    for p in rock {
        if other_rocks.contains(&Point { x: p.x, y: p.y - 1 }) {
            return true;
        }
    }
    false
}

fn print_chamber(rocks: &HashSet<Point>, falling_rock: &Vec<Point>, max_y: i64) {
    for y in 0..max_y + 1 {
        print!("|");
        for x in 0..WIDTH {
            let pnt = Point { x, y: max_y - y };
            let c = if rocks.contains(&pnt) || falling_rock.contains(&pnt) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("|");
    }
    println!("+-------+");
}

fn sol1(rocks: Vec<Vec<Point>>, wind: Vec<Point>) -> i64 {
    let reduce: i64 = 1000000000000 / 100000;
    let mut landed_rocks: HashSet<Point> = HashSet::new();
    let mut start = Point { x: 2, y: 3 };
    let mut wind_index = 0;
    for i in 0..(reduce as usize) {
        //println!("Start: {},{}", start.x, start.y);
        let mut rock = rocks[i % rocks.len()].clone();
        add(&mut rock, start);
        let mut moving = true;
        while moving {
            // print_chamber(&landed_rocks, &rock, start.y + 5);
            let wind_dir = wind[wind_index];
            wind_index = (wind_index + 1) % wind.len();
            // println!("Wind: {},{}", wind_dir.x, wind_dir.y);
            if can_move(&rock, &landed_rocks, wind_dir) {
                add(&mut rock, wind_dir);
            }
            if can_move(&rock, &landed_rocks, Point { x: 0, y: -1 }) {
                add(&mut rock, Point { x: 0, y: -1 });
            } else {
                moving = false;
            }
        }
        let mut max_y = 0;
        for p in rock {
            max_y = if p.y > max_y { p.y } else { max_y };
            landed_rocks.insert(p);
        }
        start = Point {
            x: 2,
            y: if start.y < (max_y + 4) {
                max_y + 4
            } else {
                start.y
            },
        };
        // print_chamber(&landed_rocks, &vec![], start.y + 5);
    }
    (landed_rocks
        .iter()
        .fold(0, |max, p| if max < p.y { p.y } else { max })
        + 1)
        * reduce
}

fn sol2(wind: Vec<Point>) -> i64 {
    2
}

fn main() {
    let rocks = vec![
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ],
        vec![
            Point { x: 1, y: 2 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 0 },
        ],
        vec![
            Point { x: 2, y: 2 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ],
        vec![
            Point { x: 0, y: 3 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
        ],
        vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
        ],
    ];
    let data = get_data();
    let sol = match env::var("part") {
        Ok(part) if part == "part2" => sol2(data),
        _ => sol1(rocks, data),
    };
    println!("{}", sol);
}
