use std::fs;

fn get_input() -> Vec<i32> {
    let path = "src/05/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| {
            i32::from_str_radix(
                &row.replace("F", "0")
                    .replace("B", "1")
                    .replace("R", "1")
                    .replace("L", "0"),
                2,
            )
            .unwrap()
        })
        .collect()
}

fn main() {
    let mut ids: Vec<i32> = get_input();
    ids.sort();
    println!("Fist challenge: {}", ids.last().unwrap());
    let mut i = 1;
    let place = loop {
        if ids[i] - ids[i - 1] > 1 {
            break ids[i - 1] + 1;
        }
        i += 1;
    };
    println!("Second challenge: {}", place);
}
