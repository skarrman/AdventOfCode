use std::collections::HashMap;
use std::fs;

fn get_data() -> Vec<i32> {
    let path = "src/15/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split(",")
        .map(|ch| ch.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let data = get_data();
    let (mut num, mut turn) = (data[0], 1);
    let mut hist: HashMap<i32, i32> = HashMap::new();
    for &n in data.iter().skip(1) {
        hist.insert(num, turn);
        num = n;
        turn += 1;
    }
    let mut fst = -1;
    while turn < 30000000 {
        let _num = if hist.contains_key(&num) {
            turn - hist.get(&num).unwrap()
        } else {
            0
        };
        hist.insert(num, turn);
        num = _num;
        turn += 1;
        if turn == 2020 {
            fst = num;
        }
    }
    println!("First problem: {}", fst);
    println!("Second problem: {}", num);
}
