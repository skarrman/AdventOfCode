use std::{
    collections::{HashMap, HashSet},
    env, fs, ops,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
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

const NULL_MOVE: Move = Move {
    pos: Pos { x: 0, y: 0 },
    dir: Pos { x: 0, y: 0 },
};

const UP: Pos = Pos { x: 0, y: -1 };
const RIGHT: Pos = Pos { x: 1, y: 0 };
const DOWN: Pos = Pos { x: 0, y: 1 };
const LEFT: Pos = Pos { x: -1, y: 0 };

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Move {
    pos: Pos,
    dir: Pos,
}

fn get_data() -> (HashMap<Pos, char>, Move) {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut map = HashMap::new();
    let mut start = NULL_MOVE;
    file_contents.split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '.' || c == '#' {
                map.insert(
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            } else {
                let pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                map.insert(pos, '.');
                start = Move {
                    pos,
                    dir: match c {
                        '^' => UP,
                        '>' => RIGHT,
                        'v' => DOWN,
                        '<' => LEFT,
                        _ => panic!(),
                    },
                };
            }
        })
    });
    (map, start)
}

fn walk(map: &HashMap<Pos, char>, start: Move) -> (bool, usize) {
    let mut cur = start;
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut last_visited = 0;
    let mut same_occ = 0;
    loop {
        if visited.insert(cur.pos) {
            same_occ = 0;
            last_visited = visited.len();
        }
        if last_visited == visited.len() {
            same_occ += 1;
            if same_occ >= 1 + last_visited && same_occ >= 2 {
                return (true, last_visited);
            }
        }
        let next = cur.pos + cur.dir;
        if !map.contains_key(&next) {
            break;
        }
        if map[&next] != '.' {
            cur.dir = match cur.dir {
                d if d == UP => RIGHT,
                d if d == RIGHT => DOWN,
                d if d == DOWN => LEFT,
                d if d == LEFT => UP,
                _ => panic!(),
            };
            continue;
        }
        cur.pos = next;
    }
    (false, visited.len())
}

fn find_loops(orig_map: &HashMap<Pos, char>, start: Move) -> usize {
    let mut map = orig_map.clone();
    let (len, wid) = map.keys().fold((0, 0), |(l, w), p| {
        (if l < p.y { p.y } else { l }, if w < p.x { p.x } else { w })
    });
    let mut positions = 0;
    for y in 0..=len {
        for x in 0..=wid {
            let pos = Pos { x, y };
            if map[&pos] == '#' {
                continue;
            }
            map.insert(pos, '#');
            positions += if walk(&map, start).0 { 1 } else { 0 };
            map.insert(pos, '.');
        }
    }
    positions
}

fn main() {
    let (map, start) = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", find_loops(&map, start)),
        _ => println!("{}", walk(&map, start).1),
    }
}
