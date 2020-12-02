use serde::Deserialize;
use std::fs;

//Takes input in form of a JSON file formatted as: { "data": [numbers...]}

const TARGET: i32 = 2020;

#[derive(Deserialize)]
struct Input {
    data: Vec<i32>,
}

fn get_data() -> Vec<i32> {
    let path = "src/01/input.json";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let input: Input = serde_json::from_str(&file_contents).expect("JSON could not be parsed");
    input.data
}

fn search(data: &Vec<i32>, mut i: usize, term: i32) -> i32 {
    let mut j = data.len() - 1;
    loop {
        let sum = data[i] + data[j] + term;
        if i == j {
            break 0;
        } else if sum < TARGET {
            i += 1;
        } else if sum > TARGET {
            j -= 1;
        } else {
            break data[i] * data[j];
        }
    }
}

fn first_task(data: &Vec<i32>) {
    let res = search(data, 0, 0);
    println!("First challenge {}", res);
}

fn second_task(data: &Vec<i32>) {
    let res = data
        .iter()
        .enumerate()
        .fold(0, |s, (i, &k)| s + search(data, i, k) * k);
    println!("Second challenge {}", res);
}

fn main() {
    let mut data = get_data();
    data.sort();
    first_task(&data);
    second_task(&data)
}
