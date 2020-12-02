use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Input {
    data: Vec<Password>,
}

#[derive(Deserialize)]
struct Password {
    low: usize,
    high: usize,
    letter: char,
    pwd: String,
}

fn get_data() -> Vec<Password> {
    let path = "src/02/input.json";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let input: Input = serde_json::from_str(&file_contents).expect("JSON could not be parsed");
    input.data
}

fn password_contains(pwd: &Password) -> bool {
    let count = pwd.pwd.matches(pwd.letter).count();
    count >= pwd.low && count <= pwd.high
}

fn password_position(pwd: &Password) -> bool {
    let fst = pwd.pwd.chars().nth(pwd.low - 1).unwrap();
    let snd = pwd.pwd.chars().nth(pwd.high - 1).unwrap();
    let letter = pwd.letter;
    fst == letter && snd != letter || fst != letter && snd == letter
}

fn main() {
    let passwords = get_data();
    let correct_pwds_fst = passwords.iter().fold(0, |corr, pwd| {
        corr + if password_contains(pwd) { 1 } else { 0 }
    });
    println!("Fists task: {}", correct_pwds_fst);
    let correct_pwds_snd = passwords.iter().fold(0, |corr, pwd| {
        corr + if password_position(pwd) { 1 } else { 0 }
    });
    println!("Second task: {}", correct_pwds_snd)
}
