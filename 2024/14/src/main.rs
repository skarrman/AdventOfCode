use std::fs;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Vector,
    v: Vector,
}

fn get_data() -> Vec<Robot> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split("\n")
        .map(|r| {
            let ps = r
                .split(" ")
                .map(|p| {
                    p.replace("p", "")
                        .replace("v", "")
                        .replace("=", "")
                        .split(",")
                        .map(|d| d.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .flatten()
                .collect::<Vec<i64>>();
            Robot {
                p: Vector { x: ps[0], y: ps[1] },
                v: Vector { x: ps[2], y: ps[3] },
            }
        })
        .collect()
}

fn print(robots: &Vec<Robot>, wid: i64, hei: i64) {
    for y in 0..hei {
        for x in 0..wid {
            if let Some(v) = robots
                .iter()
                .enumerate()
                .find(|(_, &p)| p.p.x == x && p.p.y == y)
            {
                print!("X");
            } else {
                print!(".")
            }
        }
        println!("");
    }
    println!("");
}

fn play(orig_robots: &Vec<Robot>, wid: i64, hei: i64, secs: usize) -> i64 {
    let mut robots = orig_robots.clone();
    print(&robots, wid, hei);
    let mut next = 23;
    for s in 1..=secs {
        let mut next_robots = Vec::new();
        for r in robots {
            next_robots.push(Robot {
                p: Vector {
                    x: ((r.p.x + r.v.x) % (wid) + (wid)) % (wid),
                    y: ((r.p.y + r.v.y) % (hei) + (hei)) % (hei),
                },
                v: r.v,
            });
        }
        robots = next_robots;
        if s == next {
            println!("{}", s);
            print(&robots, wid, hei);
            next += 101;
        }
    }
    let mut quads = vec![0, 0, 0, 0];
    for r in robots {
        match r.p {
            p if p.x < wid / 2 && p.y < hei / 2 => quads[0] += 1,
            p if p.x > wid / 2 && p.y < hei / 2 => quads[1] += 1,
            p if p.x < wid / 2 && p.y > hei / 2 => quads[2] += 1,
            p if p.x > wid / 2 && p.y > hei / 2 => quads[3] += 1,
            _ => (),
        }
    }
    quads.iter().fold(1, |prod, &n| prod * n)
}

fn main() {
    let input = get_data();

    println!("{:?}", play(&input, 101, 103, 10000));
}
