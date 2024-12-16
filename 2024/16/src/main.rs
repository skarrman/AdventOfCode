use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env, fs, i64, ops,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

const NORTH: Pos = Pos { x: 0, y: -1 };
const EAST: Pos = Pos { x: 1, y: 0 };
const SOUTH: Pos = Pos { x: 0, y: 1 };
const WEST: Pos = Pos { x: -1, y: 0 };

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: Pos,
    dir: Pos,
    tiles: HashSet<Pos>,
    score: i64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn get_data() -> (Pos, Pos, HashSet<Pos>) {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut walls = HashSet::new();
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    file_contents.split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            let p = Pos {
                x: x as i32,
                y: y as i32,
            };
            if c == '#' {
                walls.insert(p);
            } else if c == 'S' {
                start = p
            } else if c == 'E' {
                end = p;
            }
        })
    });
    (start, end, walls)
}

fn get_counterclockwise(dir: Pos) -> Pos {
    match dir {
        d if d == NORTH => WEST,
        d if d == EAST => NORTH,
        d if d == SOUTH => EAST,
        d if d == WEST => SOUTH,
        _ => panic!(),
    }
}

fn get_clockwise(dir: Pos) -> Pos {
    match dir {
        d if d == NORTH => EAST,
        d if d == EAST => SOUTH,
        d if d == SOUTH => WEST,
        d if d == WEST => NORTH,
        _ => panic!(),
    }
}

fn djikstra(start: Pos, end: Pos, walls: &HashSet<Pos>, snd: bool, best_score: i64) -> i64 {
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut tiles = HashSet::new();
    let mut best_tiles: HashSet<Pos> = HashSet::new();
    tiles.insert(start);
    queue.push(Node {
        pos: start,
        dir: EAST,
        tiles: tiles,
        score: 0,
    });
    let mut visited: HashMap<(Pos, Pos), i64> = HashMap::new();
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        if next.pos == end {
            if snd {
                if next.score == best_score {
                    best_tiles.extend(next.tiles.clone());
                }
            } else {
                return next.score;
            }
        }
        if snd
            && visited.contains_key(&((next.pos, next.dir)))
            && visited[&((next.pos, next.dir))] < next.score
            || !snd && visited.contains_key(&((next.pos, next.dir)))
        {
            continue;
        }
        visited.insert((next.pos, next.dir), next.score);
        if !walls.contains(&(next.pos + next.dir)) && next.score + 1 <= best_score {
            let mut tiles = next.tiles.clone();
            tiles.insert(next.pos + next.dir);
            queue.push(Node {
                pos: next.pos + next.dir,
                dir: next.dir,
                tiles: tiles,
                score: next.score + 1,
            });
        }
        let clockwise = get_clockwise(next.dir);
        if !walls.contains(&(next.pos + clockwise)) && next.score + 1001 <= best_score {
            let mut tiles = next.tiles.clone();
            tiles.insert(next.pos + clockwise);
            queue.push(Node {
                pos: next.pos + clockwise,
                dir: clockwise,
                tiles: tiles,
                score: next.score + 1001,
            });
        }

        let counterclockwise = get_counterclockwise(next.dir);
        if !walls.contains(&(next.pos + counterclockwise)) && next.score + 1001 <= best_score {
            let mut tiles = next.tiles.clone();
            tiles.insert(next.pos + counterclockwise);
            queue.push(Node {
                pos: next.pos + counterclockwise,
                dir: counterclockwise,
                tiles: tiles,
                score: next.score + 1001,
            });
        }
    }
    best_tiles.len() as i64
}

fn main() {
    let (start, end, walls) = get_data();
    let best_score = djikstra(start, end, &walls, false, i64::MAX);
    match env::var("part") {
        Ok(part) if part == "part2" => {
            println!("{}", djikstra(start, end, &walls, true, best_score))
        }
        _ => println!("{}", best_score),
    }
}
