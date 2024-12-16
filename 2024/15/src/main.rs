use std::{collections::HashMap, env, fs, ops};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl ops::Add<Pos> for Pos {
    fn add(self, _rhs: Pos) -> Pos {
        Pos {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }

    type Output = Pos;
}

const UP: Pos = Pos { x: 0, y: -1 };
const RIGHT: Pos = Pos { x: 1, y: 0 };
const DOWN: Pos = Pos { x: 0, y: 1 };
const LEFT: Pos = Pos { x: -1, y: 0 };

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Obj {
    Robot,
    Box,
    Wall,
    LeftBox,
    RightBox,
}

fn get_data() -> (HashMap<Pos, Obj>, Vec<Pos>) {
    let path = "src/input.txt";
    let read_to_string = fs::read_to_string(path);
    let file_contents = read_to_string.expect("Could not read file");

    let mut map = HashMap::new();
    let parts = file_contents.split("\n\n").collect::<Vec<&str>>();
    parts[0].split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                map.insert(
                    Pos {
                        x: x as i64,
                        y: y as i64,
                    },
                    match c {
                        '@' => Obj::Robot,
                        'O' => Obj::Box,
                        '#' => Obj::Wall,
                        _ => panic!(),
                    },
                );
            }
        })
    });
    let dirs = parts[1]
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!(),
        })
        .collect();
    (map, dirs)
}

fn get_data_part2() -> (HashMap<Pos, Obj>, Vec<Pos>) {
    let path = "src/input.txt";
    let read_to_string = fs::read_to_string(path);
    let file_contents = read_to_string.expect("Could not read file");

    let mut map = HashMap::new();
    let parts = file_contents.split("\n\n").collect::<Vec<&str>>();
    parts[0].split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            let cur = Pos {
                x: (x * 2) as i64,
                y: (y) as i64,
            };
            let next = Pos {
                x: (x * 2 + 1) as i64,
                y: (y) as i64,
            };
            match c {
                '@' => {
                    map.insert(cur, Obj::Robot);
                }
                '#' => {
                    map.insert(cur, Obj::Wall);
                    map.insert(next, Obj::Wall);
                }
                'O' => {
                    map.insert(cur, Obj::LeftBox);
                    map.insert(next, Obj::RightBox);
                }
                _ => (),
            };
        })
    });
    let dirs = parts[1]
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!(),
        })
        .collect();
    (map, dirs)
}

fn print_map(map: &HashMap<Pos, Obj>) {
    let (w, h) = map.keys().fold((0, 0), |(mw, my), &p| {
        (
            if p.x > mw { p.x } else { mw },
            if p.y > my { p.y } else { my },
        )
    });
    for y in 0..=h {
        for x in 0..=w {
            print!(
                "{}",
                if map.contains_key(&Pos { x, y }) {
                    match map[&Pos { x, y }] {
                        Obj::Robot => '@',
                        Obj::Wall => '#',
                        Obj::Box => 'O',
                        Obj::LeftBox => '[',
                        Obj::RightBox => ']',
                    }
                } else {
                    '.'
                }
            )
        }
        println!();
    }
    println!();
}

fn can_move(map: &HashMap<Pos, Obj>, pos: Pos, dir: Pos) -> bool {
    let next = pos + dir;
    if !map.contains_key(&next) {
        return true;
    }
    match map[&next] {
        Obj::Wall => false,
        Obj::LeftBox => {
            can_move(map, next, dir)
                && (dir == LEFT
                    || can_move(
                        map,
                        Pos {
                            x: next.x + 1,
                            y: next.y,
                        },
                        dir,
                    ))
        }
        Obj::RightBox => {
            can_move(map, next, dir)
                && (dir == RIGHT
                    || can_move(
                        map,
                        Pos {
                            x: next.x - 1,
                            y: next.y,
                        },
                        dir,
                    ))
        }
        Obj::Box => can_move(map, pos, dir),
        _ => false,
    }
}

fn try_move(map: &mut HashMap<Pos, Obj>, pos: Pos, dir: Pos) -> bool {
    let next = pos + dir;
    if !map.contains_key(&next) {
        map.insert(next, map[&pos]);
        map.remove(&pos);
        return true;
    }
    match map[&next] {
        Obj::Wall => false,
        Obj::LeftBox => {
            let rgt = Pos {
                x: next.x + 1,
                y: next.y,
            };
            if can_move(map, next, dir) && can_move(map, rgt, dir) {
                if dir == RIGHT {
                    try_move(map, rgt, dir);
                    try_move(map, next, dir);
                } else {
                    try_move(map, next, dir);
                    try_move(map, rgt, dir);
                }
                map.insert(next, map[&pos]);
                map.remove(&pos);
                true
            } else {
                false
            }
        }
        Obj::RightBox => {
            let lft = Pos {
                x: next.x - 1,
                y: next.y,
            };
            if can_move(map, next, dir) && can_move(map, lft, dir) {
                if dir == LEFT {
                    try_move(map, lft, dir);
                    try_move(map, next, dir);
                } else {
                    try_move(map, next, dir);
                    try_move(map, lft, dir);
                }
                map.insert(next, map[&pos]);
                map.remove(&pos);
                true
            } else {
                false
            }
        }
        Obj::Box => {
            if try_move(map, next, dir) {
                map.insert(next, map[&pos]);
                map.remove(&pos);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn move_robot(orig_map: &HashMap<Pos, Obj>, moves: &Vec<Pos>) -> i64 {
    let mut map = orig_map.clone();
    let mut robot = orig_map
        .iter()
        .fold(UP, |p, (k, v)| if *v == Obj::Robot { *k } else { p });
    print_map(&map);
    for m in moves {
        // println!("{:?}", *m);
        if try_move(&mut map, robot, *m) {
            robot = robot + *m;
        }
        // print_map(&map);
    }
    map.iter()
        .filter(|(_, v)| **v == Obj::Box || **v == Obj::LeftBox)
        .fold(0, |sum, (p, _)| sum + p.x + 100 * p.y)
}

fn main() {
    match env::var("part") {
        Ok(part) if part == "part2" => {
            let (map, dirs) = get_data_part2();
            println!("{}", move_robot(&map, &dirs));
        }
        _ => {
            let (map, dirs) = get_data();
            println!("{}", move_robot(&map, &dirs));
        }
    }
}
