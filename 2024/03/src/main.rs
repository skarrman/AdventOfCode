use regex::Regex;
use std::{env, fs};

fn get_data() -> String {
    let path = "src/input.txt";
    fs::read_to_string(path).expect("Could not read file")
}

fn filter_donts(input: String) -> String {
    let mut tmp = input.clone();
    loop {
        match tmp.split_once("don't()") {
            None => return tmp,
            Some((fst, snd)) => match snd.split_once("do()") {
                Some((_, snd)) => tmp = format!("{}{}", fst, snd),
                None => return String::from(fst),
            },
        }
    }
}

fn main() {
    let input = get_data();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    match env::var("part") {
        Ok(part) if part == "part2" => println!(
            "{}",
            re.captures_iter(filter_donts(input).as_str())
                .fold(0, |sum, cap| sum
                    + &cap[1].parse::<i32>().unwrap()
                        * &cap[2].parse::<i32>().unwrap())
        ),
        _ => println!(
            "{}",
            re.captures_iter(input.as_str()).fold(0, |sum, cap| sum
                + &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap())
        ),
    }
}
