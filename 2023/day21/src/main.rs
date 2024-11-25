use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_data() -> (Point, HashSet<Point>) {
    let mut map = HashSet::new();
    let mut start = Point { x: 0, y: 0 };
    for (y, row) in fs::read_to_string("input.txt")
        .expect("Could not read file")
        .split("\n")
        .enumerate()
    {
        for (x, char) in row.chars().enumerate() {
            match char {
                '.' => map.insert(Point {
                    x: x as i32,
                    y: y as i32,
                }),
                'S' => {
                    start = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    map.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    })
                }
                _ => true,
            };
        }
    }
    (start, map)
}

fn find_path(start: Point, map: HashSet<Point>, steps: i32, size: i32) -> i32 {
    let mut places: HashSet<Point> = HashSet::from([start]);
    let dir = vec![
        Point { x: 0, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
    ];
    // let mut last = 0;
    // let mut last_diff = 1;
    let half = (size - 1) / 2;
    for i in 1..(size * 2 + half) {
        let mut next_places: HashSet<Point> = HashSet::new();
        // if (i % 100) == 0 {
        // last = places.len() as i32;
        // }
        for place in places {
            for d in &dir {
                let point = Point {
                    x: place.x + d.x,
                    y: place.y + d.y,
                };
                let reduced = Point {
                    x: ((point.x % size) + size) % size,
                    y: ((point.y % size) + size) % size,
                };
                if map.contains(&reduced) {
                    next_places.insert(point);
                }
            }
        }
        if ((i - half) % size) == 0 {
            println!("Steps: {}, Reached: {}", i, next_places.len());
        }

        // }
        places = next_places;
    }
    places.len() as i32
}

fn main() {
    let (start, map) = get_data();
    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", find_path(start, map, 64, 11)),
        _ => println!("{}", find_path(start, map, 5000, 11)),
    }
}
