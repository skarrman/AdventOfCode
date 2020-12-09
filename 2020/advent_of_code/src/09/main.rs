use std::fs;

const PREABLE: usize = 25;

fn get_data() -> Vec<i64> {
    let path = "src/09/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

fn is_valid(mut data: Vec<i64>, target: i64) -> bool {
    data.sort();
    let (mut i, mut j) = (0, data.len() - 1);
    loop {
        let sum = data[i] + data[j];
        if i == j {
            break false;
        } else if sum < target {
            i += 1;
        } else if sum > target {
            j -= 1;
        } else {
            break true;
        }
    }
}

fn main() {
    let data = get_data();
    let mut fst_num = -1;
    for i in PREABLE + 1..data.len() {
        if !is_valid(data[i - PREABLE - 1..i].to_vec(), data[i]) {
            fst_num = data[i];
            break;
        }
    }
    println!("First problem: {}", fst_num);
    let (mut min, mut max) = (0, 0);
    let mut found = false;
    let mut i = 0;
    while !found {
        min = data[i];
        max = data[i];
        let mut sum = 0;
        for j in i..data.len() {
            let d_j = data[j];
            if d_j < min {
                min = d_j;
            } else if d_j > max {
                max = d_j;
            };
            sum += d_j;
            if sum == fst_num {
                found = true;
                break;
            }
        }
        i += 1;
    }
    println!("Second problem: {}", min + max);
}
