use std::fs;

fn get_data() -> Vec<i32> {
    let path = "src/10/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let mut data = get_data();
    data.sort();
    data.push(data[data.len() - 1] + 3);
    data.push(0);
    data.rotate_right(1);
    let (mut d_1, mut d_3) = (0, 0);
    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];
        if diff == 1 {
            d_1 += 1;
        } else if diff == 3 {
            d_3 += 1;
        }
    }
    println!("First challenge: {}", d_1 * d_3);
    let mut paths: Vec<i64> = vec![1; data.len()];
    for i in (0..=data.len() - 2).rev() {
        let mut ps = 0;
        for d in 1..=3 {
            if i + d < data.len() && data[i + d] - data[i] <= 3 {
                ps += paths[i + d];
            }
        }
        paths[i] = ps;
    }
    println!("Second challenge: {}", paths[0]);
}
