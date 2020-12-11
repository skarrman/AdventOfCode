use std::fs;

type Places = Vec<Vec<char>>;

const OCC: char = '#';
const FREE: char = 'L';

fn get_data() -> Places {
    let path = "src/11/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| row.chars().collect())
        .collect()
}

fn print_data(data: &Places) {
    println!("Data: ({},{})", data.len(), data[0].len());
    for row in data {
        let mut row_str = "".to_string();
        for c in row {
            row_str = format!("{}{}", row_str, c);
        }
        println!("{}", row_str);
    }
    println!();
}

fn is_valid(data: &Places, i: usize, j: usize) -> bool {
    i < data.len() && j < data[i].len()
}

fn direction_is_occupied(
    data: &Places,
    mut i: i32,
    mut j: i32,
    d_i: i32,
    d_j: i32,
    depth: i32,
) -> bool {
    i += d_i;
    j += d_j;
    let mut d = 1;
    while (depth == -1 || d <= depth) && is_valid(data, i as usize, j as usize) {
        match data[i as usize][j as usize] {
            OCC => return true,
            FREE => return false,
            _ => (),
        }
        i += d_i;
        j += d_j;
        d += 1;
    }
    false
}

fn occupied_adjacent(data: &Places, i: i32, j: i32, depth: i32) -> usize {
    let mut occupied = 0;
    for d_i in -1..=1 {
        for d_j in -1..=1 {
            occupied +=
                if !(d_i == 0 && d_j == 0) && direction_is_occupied(data, i, j, d_i, d_j, depth) {
                    1
                } else {
                    0
                };
        }
    }
    occupied
}

fn run(mut data: Places, th: usize, depth: i32) -> i32 {
    let mut stable = false;
    while !stable {
        stable = true;
        let mut _data = data.clone();
        for (i, row) in data.iter().enumerate() {
            for (j, &place) in row.iter().enumerate() {
                if place != '.' {
                    let o_a = occupied_adjacent(&data, i as i32, j as i32, depth);
                    _data[i][j] = match (place, o_a) {
                        (FREE, 0) => {
                            stable = false;
                            OCC
                        }
                        (OCC, o) if o >= th => {
                            stable = false;
                            FREE
                        }
                        (p, _) => p,
                    }
                }
            }
        }
        data = _data;
    }
    data.iter().fold(0, |f, row| {
        f + row
            .iter()
            .fold(0, |_f, &p| _f + if p == OCC { 1 } else { 0 })
    })
}

fn main() {
    let data = get_data();
    println!("First problem: {}", run(data.clone(), 4, 1));
    println!("Second problem: {}", run(data.clone(), 5, -1));
}
