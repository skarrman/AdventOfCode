use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn get_data() -> (i32, i32, HashMap<char, Vec<Pos>>) {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents.split("\n").enumerate().fold(
        (0, 0, HashMap::new()),
        |(mut w, h, mut map), (y_u, row)| {
            let y = y_u as i32;
            row.chars().enumerate().for_each(|(x_u, c)| {
                let x = x_u as i32;
                if c != '.' {
                    let pos = Pos { x, y };
                    map.entry(c).or_insert(Vec::new()).push(pos);
                }
                w = if x > w { x } else { w };
            });
            (w, if y > h { y } else { h }, map)
        },
    )
}

fn is_valid(pos: Pos, width: i32, height: i32) -> bool {
    0 <= pos.x && pos.x <= width && 0 <= pos.y && pos.y <= height
}

fn find_antinodes(
    antinodes: &mut HashSet<Pos>,
    width: i32,
    height: i32,
    antennas: &Vec<Pos>,
    follow: bool,
) {
    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            let mut k = 0;
            let mut any_valid = true;
            while any_valid {
                any_valid = false;
                let p1 = antennas[i];
                let p2 = antennas[j];
                let a1 = Pos {
                    x: p1.x - k * (p2.x - p1.x),
                    y: p1.y - k * (p2.y - p1.y),
                };
                let a2 = Pos {
                    x: p2.x - k * (p1.x - p2.x),
                    y: p2.y - k * (p1.y - p2.y),
                };
                if is_valid(a1, width, height) {
                    antinodes.insert(a1);
                    any_valid = true;
                }
                if is_valid(a2, width, height) {
                    antinodes.insert(a2);
                    any_valid = true;
                }

                k += 1;
                if !follow {
                    break;
                }
            }
        }
    }
}

fn find_all_antinodes(
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Pos>>,
    follow: bool,
) -> usize {
    let mut antinodes = HashSet::new();
    antennas
        .iter()
        .for_each(|(_, a)| find_antinodes(&mut antinodes, width, height, a, follow));
    antinodes.len()
}

fn main() {
    let (width, height, antennas) = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => {
            println!("{}", find_all_antinodes(width, height, antennas, true))
        }
        _ => println!("{}", find_all_antinodes(width, height, antennas, false)),
    }
}
