use std::{collections::HashMap, env, fs};

fn get_data() -> Vec<u64> {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split(" ")
        .map(|row| row.parse::<u64>().unwrap())
        .collect()
}

fn blink(orig_stones: &Vec<u64>, times: usize) -> usize {
    let mut stones: HashMap<u64, usize> = orig_stones.iter().map(|s| (*s, 1usize)).collect();
    for _ in 0..times {
        let mut next_stones = HashMap::new();
        for (s, n) in stones {
            match s {
                0 => {
                    next_stones.entry(1).and_modify(|m| *m += n).or_insert(n);
                }
                x if (format!("{x}").len() % 2) == 0 => {
                    let dig = format!("{x}");
                    let (fst, snd) = dig.split_at(dig.len() / 2);
                    next_stones
                        .entry(fst.parse().unwrap())
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                    next_stones
                        .entry(snd.parse().unwrap())
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                }
                x => {
                    next_stones
                        .entry(x * 2024)
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                }
            };
        }

        stones = next_stones;
    }

    stones.iter().fold(0, |sum, (_, n)| sum + n)
}

fn main() {
    let input = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => println!("{}", blink(&input, 75)),
        _ => println!("{}", blink(&input, 25)),
    }
}
