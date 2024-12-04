use std::{collections::HashMap, env, fs, ops};

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

struct Node {
    next: Pos,
    dir: Pos,
    next_char: char,
}

fn get_data() -> HashMap<Pos, char> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut map = HashMap::new();
    file_contents.split("\n").enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        })
    });
    map
}

fn find_x_mas(map: &HashMap<Pos, char>, node: Node) -> bool {
    let top_left = node.next + Pos { x: -1, y: -1 };
    let bottom_right = node.next + Pos { x: 1, y: 1 };
    let bottom_left = node.next + Pos { x: -1, y: 1 };
    let top_right = node.next + Pos { x: 1, y: -1 };

    map.contains_key(&top_left)
        && map.contains_key(&bottom_right)
        && map.contains_key(&bottom_left)
        && map.contains_key(&top_right)
        && (map[&top_left] == 'M' && map[&bottom_right] == 'S'
            || map[&top_left] == 'S' && map[&bottom_right] == 'M')
        && (map[&bottom_left] == 'M' && map[&top_right] == 'S'
            || map[&bottom_left] == 'S' && map[&top_right] == 'M')
}

fn find_xmas(map: &HashMap<Pos, char>, node: Node) -> bool {
    let next = node.next + node.dir;
    if !map.contains_key(&next) || map[&next] != node.next_char {
        return false;
    }
    match node.next_char {
        'M' => find_xmas(
            map,
            Node {
                next,
                dir: node.dir,
                next_char: 'A',
            },
        ),
        'A' => find_xmas(
            map,
            Node {
                next,
                dir: node.dir,
                next_char: 'S',
            },
        ),
        'S' => true,
        _ => panic!(),
    }
}

fn main() {
    let map = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!(
            "{}",
            map.iter()
                .filter(|(_, v)| **v == 'A')
                .fold(0, |sum, (k, _)| sum
                    + if find_x_mas(
                        &map,
                        Node {
                            next: *k,
                            dir: Pos { x: 0, y: 0 },
                            next_char: 'X'
                        }
                    ) {
                        1
                    } else {
                        0
                    })
        ),
        _ => println!(
            "{}",
            map.iter()
                .filter(|(_, v)| **v == 'X')
                .fold(0, |words, (k, _)| words
                    + vec![
                        Pos { x: -1, y: -1 },
                        Pos { x: 0, y: -1 },
                        Pos { x: 1, y: -1 },
                        Pos { x: 1, y: 0 },
                        Pos { x: 1, y: 1 },
                        Pos { x: 0, y: 1 },
                        Pos { x: -1, y: 1 },
                        Pos { x: -1, y: 0 },
                    ]
                    .iter()
                    .fold(0, |sum, dir| sum
                        + if find_xmas(
                            &map,
                            Node {
                                next: *k,
                                dir: *dir,
                                next_char: 'M'
                            }
                        ) {
                            1
                        } else {
                            0
                        }))
        ),
    }
}
