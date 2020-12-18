use std::collections::HashMap;
use std::fs;
const ACTIVE: char = '#';
const INACTIVE: char = '.';

#[derive(Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl Eq for Point {}

fn get_data() -> HashMap<Point, char> {
    let path = "src/17/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let zw = 0;
    let mut map = HashMap::new();
    for (y, row) in file_contents.split("\n").enumerate() {
        for (x, c) in row.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                    z: zw,
                    w: zw,
                },
                c,
            );
        }
    }
    map
}

fn get_ranges(map: &HashMap<Point, char>) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    map.keys().fold(
        (0, 0, 0, 0, 0, 0, 0, 0),
        |(min_x, max_x, min_y, max_y, min_z, max_z, min_w, max_w), p| {
            (
                if p.x < min_x { p.x } else { min_x },
                if p.x > max_x { p.x } else { max_x },
                if p.y < min_y { p.y } else { min_y },
                if p.y > max_y { p.y } else { max_y },
                if p.z < min_z { p.z } else { min_z },
                if p.z > max_z { p.z } else { max_z },
                if p.w < min_w { p.w } else { min_w },
                if p.w > max_w { p.w } else { max_w },
            )
        },
    )
}

fn active_neighbours(
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    forth_dim: bool,
    map: &HashMap<Point, char>,
) -> i32 {
    let d: Vec<i32> = vec![-1, 0, 1];
    let dws = if forth_dim { d.clone() } else { vec![0] };
    let mut active_n = 0;
    for &dx in &d {
        for &dy in &d {
            for &dz in &d {
                for &dw in &dws {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        active_n += if *map
                            .get(&Point {
                                x: x + dx,
                                y: y + dy,
                                z: z + dz,
                                w: w + dw,
                            })
                            .unwrap_or(&INACTIVE)
                            == ACTIVE
                        {
                            1
                        } else {
                            0
                        };
                    }
                }
            }
        }
    }
    active_n
}

fn main() {
    let mut map = get_data();
    for _ in 0..6 {
        let mut _map = HashMap::new();
        let (min_x, max_x, min_y, max_y, min_z, max_z, _, _) = get_ranges(&map);
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    _map.insert(
                        Point {
                            x: x,
                            y: y,
                            z: z,
                            w: 0,
                        },
                        match (
                            *map.get(&Point {
                                x: x,
                                y: y,
                                z: z,
                                w: 0,
                            })
                            .unwrap_or(&INACTIVE),
                            active_neighbours(x, y, z, 0, false, &map),
                        ) {
                            (INACTIVE, 3) => ACTIVE,
                            (ACTIVE, n) if n == 2 || n == 3 => ACTIVE,
                            _ => INACTIVE,
                        },
                    );
                }
            }
        }
        map = _map;
    }
    println!(
        "First problem: {}",
        map.iter()
            .fold(0, |sum, (_, &c)| sum + if c == ACTIVE { 1 } else { 0 })
    );

    let mut map = get_data();
    for _ in 0..6 {
        let mut _map = HashMap::new();
        let (min_x, max_x, min_y, max_y, min_z, max_z, min_w, max_w) = get_ranges(&map);
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    for w in (min_w - 1)..=(max_w + 1) {
                        _map.insert(
                            Point {
                                x: x,
                                y: y,
                                z: z,
                                w: w,
                            },
                            match (
                                *map.get(&Point {
                                    x: x,
                                    y: y,
                                    z: z,
                                    w: w,
                                })
                                .unwrap_or(&INACTIVE),
                                active_neighbours(x, y, z, w, true, &map),
                            ) {
                                (INACTIVE, 3) => ACTIVE,
                                (ACTIVE, n) if n == 2 || n == 3 => ACTIVE,
                                _ => INACTIVE,
                            },
                        );
                    }
                }
            }
        }
        map = _map;
    }
    println!(
        "Second problem: {}",
        map.iter()
            .fold(0, |sum, (_, &c)| sum + if c == ACTIVE { 1 } else { 0 })
    );
}
