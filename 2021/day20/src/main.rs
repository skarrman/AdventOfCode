use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Hash, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn get_data() -> (Vec<i32>, HashMap<Point, i32>) {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let alg = parts[0]
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let pic = parts[1]
        .split("\n")
        .enumerate()
        .fold(HashMap::new(), |map, (y, row)| {
            row.chars().enumerate().fold(map, |mut _map, (x, c)| {
                _map.insert(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    if c == '#' { 1 } else { 0 },
                );
                for dy in vec![-1, 0, 1] {
                    for dx in vec![-1, 0, 1] {
                        _map.entry(Point {
                            x: x as i64 + dx,
                            y: y as i64 + dy,
                        })
                        .or_insert(0);
                    }
                }
                _map
            })
        });
    (alg, pic)
}

fn main() {
    let times = match env::var("part") {
        Ok(val) if val == "part2" => 50,
        _ => 2,
    };
    let (alg_str, mut pic) = get_data();
    let flip = (0..9).fold(0usize, |val, _| val.rotate_left(1) | alg_str[0] as usize);
    for i in 0..times {
        let mut next = HashMap::new();
        for (p, _) in &pic {
            let mut val = 0i32;
            for y in vec![-1, 0, 1] {
                for x in vec![-1, 0, 1] {
                    let pnt = Point {
                        x: p.x + x,
                        y: p.y + y,
                    };
                    val = val.rotate_left(1)
                        | if pic.contains_key(&pnt) {
                            pic[&pnt]
                        } else {
                            next.insert(
                                Point {
                                    x: p.x + x,
                                    y: p.y + y,
                                },
                                alg_str[if i % 2 == 0 { 0 } else { flip }],
                            );
                            alg_str[if i % 2 == 1 { 0 } else { flip }]
                        };
                }
            }
            next.insert(p.clone(), alg_str[val as usize]);
        }
        pic = next;
    }
    println!("{}", pic.iter().fold(0, |sum, (_, v)| sum + v))
}
