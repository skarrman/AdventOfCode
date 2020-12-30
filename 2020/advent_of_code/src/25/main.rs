use std::fs;

const DIVIDER: i64 = 20201227;

fn get_data() -> (i64, i64) {
    let path = "src/25/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let keys: Vec<i64> = file_contents
        .split("\n")
        .map(|row| row.parse().unwrap())
        .collect();
    (keys[0], keys[1])
}

fn recover_loop_size(key: i64, subject_number: i64) -> i64 {
    let mut val = 1;
    let mut i = 0;
    while val != key {
        val *= subject_number;
        val %= DIVIDER;
        i += 1;
    }
    i
}

fn transform(subject_number: i64, iterations: i64) -> i64 {
    let mut val = 1;
    for _ in 0..iterations {
        val *= subject_number;
        val %= DIVIDER;
    }
    val
}

fn main() {
    let (key1, key2) = get_data();
    let (loop1, loop2) = (recover_loop_size(key1, 7), recover_loop_size(key2, 7));
    let (enc1, enc2) = (transform(key2, loop1), transform(key1, loop2));
    assert_eq!(enc1, enc2);
    println!("First problem: {}", enc2);
    println!("Woho! All problems solved!");
}
