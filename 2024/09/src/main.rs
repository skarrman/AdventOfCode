use std::{collections::HashSet, env, fs};

fn get_data() -> Vec<i128> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let mut mem_id = 0;
    let mut is_mem = true;
    let mut data = Vec::new();
    for n in file_contents
        .chars()
        .map(|c| format!("{}", c).parse::<i128>().unwrap())
    {
        for _ in 0..n {
            data.push(if is_mem { mem_id } else { -1 });
        }
        mem_id = if is_mem { mem_id + 1 } else { mem_id };
        is_mem = !is_mem;
    }
    data
}

fn move_file_blocks(orig_data: &Vec<i128>) -> Vec<i128> {
    let mut data = orig_data.clone();
    let (mut i, mut j) = (0, data.len() - 1);
    while i < j {
        while data[i] != -1 {
            i += 1;
        }
        if i >= j {
            break;
        }
        while data[j] == -1 {
            j -= 1;
        }
        data[i] = data[j];
        data[j] = -1;
    }
    data
}

fn move_files(orig_data: &Vec<i128>) -> Vec<i128> {
    let mut data = orig_data.clone();
    let mut j = data.len() - 1;
    let mut moved: HashSet<i128> = HashSet::new();
    while j != 0 {
        while data[j] == -1 {
            if j == 0 {
                break;
            }
            j -= 1;
        }

        let val = data[j];
        let mut size = 0;
        while data[j] == val {
            if j == 0 {
                break;
            }
            j -= 1;
            size += 1;
        }

        if moved.contains(&val) {
            continue;
        }
        moved.insert(val);

        let mut i = 0;
        while i < j {
            let mut target_size = 0;
            while data[i] != -1 {
                i += 1;
            }
            while data[i] == -1 {
                i += 1;
                target_size += 1;
            }
            if i > j + 1 {
                break;
            }
            if target_size >= size {
                for k in 0..size {
                    data[i - target_size + k] = data[j + k + 1];
                    data[j + k + 1] = -1;
                }
                break;
            }
        }
    }

    data
}

fn calculate_checksum(data: &Vec<i128>) -> i128 {
    data.iter().enumerate().fold(0, |sum, (i, n)| {
        sum + if *n == -1 { 0 } else { *n * (i as i128) }
    })
}

fn main() {
    let data = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", calculate_checksum(&move_files(&data))),
        _ => println!("{}", calculate_checksum(&move_file_blocks(&data))),
    }
}
