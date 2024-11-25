use std::env;
use std::fs;

fn get_data() -> Vec<usize> {
    let mut sums = fs::read_to_string("input.txt")
        .expect("Could not read file")
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .fold(0, |s, n| s + n.parse::<usize>().unwrap())
        })
        .collect::<Vec<usize>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums
}

fn main() {
    let data = get_data();
    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", data[0] + data[1] + data[2]),
        _ => println!("{}", data[0]),
    }
}
