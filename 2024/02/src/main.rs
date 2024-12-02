use std::{env, fs};

fn get_data() -> Vec<Vec<i32>> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split("\n")
        .map(|row| row.split(" ").map(|d| d.parse::<i32>().unwrap()).collect())
        .collect()
}

fn check(row: &Vec<i32>) -> i32 {
    let inc = row[0] < row[1];
    if row
        .iter()
        .skip(1)
        .fold((true, row[0]), |(b, last), cur| match inc {
            true if last < *cur && (last - cur).abs() <= 3 => (b && true, *cur),
            false if last > *cur && (last - cur).abs() <= 3 => (b && true, *cur),
            _ => (false, *cur),
        })
        .0
    {
        1
    } else {
        0
    }
}

fn check_with_skip(row: &Vec<i32>) -> i32 {
    if check(row) == 1 {
        1
    } else {
        for i in 0..row.len() {
            let mut _row = row.clone();
            _row.remove(i);
            if check(&_row) == 1 {
                return 1;
            }
        }
        0
    }
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!(
            "{}",
            input
                .iter()
                .fold(0, |sec, seq| { sec + check_with_skip(seq) })
        ),
        _ => println!("{}", input.iter().fold(0, |sec, seq| { sec + check(seq) })),
    }
}
