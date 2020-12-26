use std::collections::HashMap;
use std::fs;

fn get_data() -> Vec<HashMap<Dir, usize>> {
    let path = "src/24/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| {
            let mut dirs = HashMap::new();
            let mut it = row.chars();
            loop {
                match it.next() {
                    Some(d) => {
                        let dir = match d {
                            'e' => Dir::East,
                            'w' => Dir::West,
                            's' => match it.next() {
                                Some('e') => Dir::Southeast,
                                _ => Dir::Southwest,
                            },
                            'n' => match it.next() {
                                Some('e') => Dir::Northeast,
                                _ => Dir::Northwest,
                            },
                            _ => break dirs,
                        };
                        let val = dirs.entry(dir).or_insert(0);
                        *val += 1;
                    }
                    None => break dirs,
                }
            }
        })
        .collect()
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
enum Dir {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn hashmap_tov_vec(dirs: HashMap<Dir, usize>) -> Vec<Dir> {
    let mut dir_vec = Vec::new();
    for (k, v) in dirs {
        for _ in 0..v {
            dir_vec.push(k.clone());
        }
    }
    dir_vec.sort();
    dir_vec
}

fn dir_to_string(dir: Dir) -> String {
    match dir {
        Dir::East => "e",
        Dir::Southeast => "se",
        Dir::Southwest => "sw",
        Dir::West => "w",
        Dir::Northwest => "nw",
        Dir::Northeast => "ne",
    }
    .to_string()
}
fn to_string(dirs: HashMap<Dir, usize>) -> String {
    let sorted_dir_vec = hashmap_tov_vec(dirs);
    let mut s = " ".to_string();
    for dir in sorted_dir_vec {
        s = format!("{} {}", s, dir_to_string(dir));
    }
    s
}

fn to_map(path: String) -> HashMap<Dir, usize> {
    path.split(" ")
        .filter(|ds| *ds != "")
        .fold(HashMap::new(), |mut dirs, ds| {
            let d = match ds {
                "e" => Dir::East,
                "se" => Dir::Southeast,
                "sw" => Dir::Southwest,
                "w" => Dir::West,
                "nw" => Dir::Northwest,
                "ne" => Dir::Northeast,
                _ => panic!("Non-exhaustive pattern in \"to_map\""),
            };
            let v = dirs.entry(d).or_insert(0);
            *v += 1;
            dirs
        })
}

fn reduce(dirs: &mut HashMap<Dir, usize>, d1: Dir, d2: Dir, to: Dir) -> bool {
    match (dirs.get(&d1), dirs.get(&d2), dirs.get(&to)) {
        (Some(v1), Some(v2), ov) if *v1 != 0 && *v2 != 0 => {
            let v = ov.unwrap_or(&0);
            let (_v1, _v2, _v) = if v1 >= v2 {
                (v1 - v2, 0, v + v2)
            } else {
                (0, v2 - v1, v + v1)
            };
            dirs.insert(d1, _v1);
            dirs.insert(d2, _v2);
            dirs.insert(to, _v);
            true
        }
        _ => false,
    }
}

fn cancel(dirs: &mut HashMap<Dir, usize>, d1: Dir, d2: Dir) -> bool {
    match (dirs.get(&d1), dirs.get(&d2)) {
        (Some(v1), Some(v2)) if *v1 != 0 && *v2 != 0 => {
            let (_v1, _v2) = if v1 >= v2 { (v1 - v2, 0) } else { (0, v2 - v1) };
            dirs.insert(d1, _v1);
            dirs.insert(d2, _v2);
            true
        }
        _ => false,
    }
}

fn minimize(mut dirs: HashMap<Dir, usize>) -> HashMap<Dir, usize> {
    let mut changed = true;
    while changed {
        changed = false;
        changed |= reduce(&mut dirs, Dir::Southeast, Dir::West, Dir::Southwest);
        changed |= reduce(&mut dirs, Dir::Southwest, Dir::East, Dir::Southeast);
        changed |= reduce(&mut dirs, Dir::Northeast, Dir::West, Dir::Northwest);
        changed |= reduce(&mut dirs, Dir::Northwest, Dir::East, Dir::Northeast);
        changed |= cancel(&mut dirs, Dir::Northwest, Dir::Southeast);
        changed |= cancel(&mut dirs, Dir::Northeast, Dir::Southwest);
        changed |= reduce(&mut dirs, Dir::Northwest, Dir::Southwest, Dir::West);
        changed |= reduce(&mut dirs, Dir::Northeast, Dir::Southeast, Dir::East);
        changed |= cancel(&mut dirs, Dir::West, Dir::East);
    }
    dirs
}

const BLACK: char = 'B';
const WHITE: char = 'W';
const DIRS: &[Dir] = &[
    Dir::East,
    Dir::Southeast,
    Dir::Southwest,
    Dir::West,
    Dir::Northwest,
    Dir::Northeast,
];

fn count_tiles(paths: &HashMap<String, char>) -> (usize, usize) {
    paths.iter().fold(
        (0, 0),
        |(b, w), (_, c)| if *c == WHITE { (b, w + 1) } else { (b + 1, w) },
    )
}

fn main() {
    let mut data = get_data();
    data = data.iter().map(|dirs| minimize(dirs.clone())).collect();
    let mut paths = HashMap::new();
    for row in data {
        let key = to_string(row.clone());
        let c = paths.entry(key).or_insert(WHITE);
        *c = if *c == WHITE { BLACK } else { WHITE };
    }

    let (black, white) = count_tiles(&paths);
    println!("First problem: black: {}  white: {}", black, white);
    paths = paths
        .iter()
        .filter(|(_, c)| *c == &BLACK)
        .map(|(k, c)| (k.clone(), *c))
        .collect();
    for (k, _) in &paths.clone() {
        let dirs = to_map(k.to_string());
        for d in DIRS {
            let mut _dirs = dirs.clone();
            let v = _dirs.entry(d.clone()).or_insert(0);
            *v += 1;
            let p = to_string(minimize(_dirs));
            if !paths.contains_key(&p) {
                paths.insert(p, WHITE);
            }
        }
    }
    for i in 1..=100 {
        let mut _paths = paths.clone();
        for (k, c) in &paths {
            let dirs = to_map(k.to_string());
            let mut black = 0;
            for d in DIRS {
                let mut _dirs = dirs.clone();
                let v = _dirs.entry(d.clone()).or_insert(0);
                *v += 1;
                let p = to_string(minimize(_dirs));
                if paths.contains_key(&p) {
                    black += if paths[&p] == BLACK { 1 } else { 0 };
                } else {
                    _paths.insert(p, WHITE);
                }
            }
            match (c, black) {
                (&BLACK, b) if b == 0 || b > 2 => {
                    _paths.insert(k.to_string(), WHITE);
                }
                (&WHITE, 2) => {
                    _paths.insert(k.to_string(), BLACK);
                }
                _ => (),
            }
        }
        paths = _paths // To reduce the amount of white tiles to consider
            .iter()
            .filter(|(k, c)| {
                *c == &BLACK || {
                    let dirs = to_map(k.to_string());
                    let mut black = 0;
                    for d in DIRS {
                        let mut _dirs = dirs.clone();
                        let v = _dirs.entry(d.clone()).or_insert(0);
                        *v += 1;
                        let p = to_string(minimize(_dirs));
                        if _paths.contains_key(&p) {
                            black += if _paths[&p] == BLACK { 1 } else { 0 };
                        }
                    }
                    black != 0
                }
            })
            .map(|(k, c)| (k.clone(), *c))
            .collect();
    }
    let (black, white) = count_tiles(&paths);
    println!("Second problem: black: {}  white: {}", black, white);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_string() {
        let dirs: HashMap<Dir, usize> = [
            (Dir::Southeast, 2),
            (Dir::Southwest, 2),
            (Dir::East, 3),
            (Dir::Northeast, 1),
        ]
        .iter()
        .cloned()
        .collect();
        let path = to_string(dirs.clone());
        let _dirs = to_map(path);
        assert_eq!(dirs, _dirs);
    }

    #[test]
    fn test_minimize() {
        let dirs: HashMap<Dir, usize> = [
            (Dir::Southeast, 2),
            (Dir::Southwest, 2),
            (Dir::East, 3),
            (Dir::Northeast, 1),
            (Dir::West, 5),
        ]
        .iter()
        .cloned()
        .collect();
        let _dirs = minimize(dirs);
        assert_eq!(_dirs.clone(), minimize(_dirs));
    }
}
