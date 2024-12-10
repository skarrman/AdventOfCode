use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    pos: Pos,
    height: i32,
}

fn get_data() -> HashMap<Pos, i32> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut map = HashMap::new();
    file_contents.split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                map.insert(
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    format!("{c}").parse::<i32>().unwrap(),
                );
            }
        })
    });
    map
}

fn bfs(map: &HashMap<Pos, i32>, start: Pos, distinct: bool) -> i32 {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(Node {
        pos: start,
        height: 0,
    });
    let mut paths = 0;

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if !distinct && visited.contains(&cur.pos) {
            continue;
        }
        visited.insert(cur.pos);

        if cur.height == 9 {
            paths += 1;
            continue;
        }

        for d in vec![
            Pos { x: -1, y: 0 },
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 1 },
        ] {
            let next = cur.pos + d;
            if !map.contains_key(&next) || map[&next] != cur.height + 1 {
                continue;
            }
            queue.push_back(Node {
                pos: next,
                height: cur.height + 1,
            });
        }
    }

    paths
}

fn find_trailhead_scores(map: &HashMap<Pos, i32>) -> i32 {
    map.into_iter()
        .filter(|(_, v)| **v == 0)
        .fold(0, |sum, (p, _)| sum + bfs(map, *p, false))
}

fn find_trailhead_ratings(map: &HashMap<Pos, i32>) -> i32 {
    map.into_iter()
        .filter(|(_, v)| **v == 0)
        .fold(0, |sum, (p, _)| sum + bfs(map, *p, true))
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", find_trailhead_ratings(&input)),
        _ => println!("{}", find_trailhead_scores(&input)),
    }
}
