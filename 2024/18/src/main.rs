use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env, fs, ops,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: Pos,
    steps: i32,
    // visited: HashSet<Pos>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn get_data() -> Vec<Pos> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut bytes = Vec::new();
    file_contents.split("\n").for_each(|row| {
        let nums = row
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        bytes.push(Pos {
            x: nums[0],
            y: nums[1],
        });
    });
    bytes
}

fn directions() -> Vec<Pos> {
    vec![
        Pos { x: -1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: 1 },
    ]
}

fn djikstra(map: &HashMap<Pos, bool>) -> i32 {
    let end = Pos {
        x: MAP_SIZE,
        y: MAP_SIZE,
    };
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    queue.push(Node {
        pos: Pos { x: 0, y: 0 },
        steps: 0,
        // visited: HashSet::new(),
    });
    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        if cur.pos == end {
            return cur.steps;
        }
        if visited.contains(&cur.pos) {
            continue;
        }
        visited.insert(cur.pos);
        for d in directions() {
            let next = cur.pos + d;
            if map.contains_key(&next) && map[&next] {
                queue.push(Node {
                    pos: next,
                    steps: cur.steps + 1,
                });
            }
        }
    }
    -1
}

fn create_map() -> HashMap<Pos, bool> {
    let mut map = HashMap::new();
    for x in 0..=MAP_SIZE {
        for y in 0..=MAP_SIZE {
            map.insert(Pos { x, y }, true);
        }
    }
    map
}

fn fill_map(
    orig_map: HashMap<Pos, bool>,
    falling_bytes: &Vec<Pos>,
    bytes: usize,
) -> HashMap<Pos, bool> {
    let mut map = orig_map.clone();
    for i in 0..bytes {
        map.insert(falling_bytes[i], false);
    }
    map
}
const MAP_SIZE: i32 = 70;
const BYTES: usize = 1024;

fn find_blocking_byte(falling_bytes: &Vec<Pos>) -> Pos {
    let mut min = 0;
    let mut max = falling_bytes.len();
    loop {
        let next = min + (max - min) / 2;
        if min == max {
            return falling_bytes[min - 1];
        }
        let steps = djikstra(&fill_map(create_map(), falling_bytes, next));
        if steps == -1 {
            max = next;
        } else {
            min = next + 1;
        }
    }
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!("{:?}", find_blocking_byte(&input)),
        _ => println!("{}", djikstra(&fill_map(create_map(), &input, BYTES))),
    }
}
