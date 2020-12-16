use std::collections::HashMap;
use std::fs;
extern crate regex;

fn parse(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn get_data() -> (
    HashMap<String, (i32, i32, i32, i32)>,
    Vec<i64>,
    Vec<Vec<i32>>,
) {
    let path = "src/16/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let chunks = file_contents
        .split("\n\n")
        .map(|chunk| chunk.to_string())
        .collect::<Vec<String>>();
    let range_re = regex::Regex::new(r"(?P<key>[a-z ]*):\s(?P<fst_min>[0-9]*)-(?P<fst_max>[0-9]*)\sor\s(?P<snd_min>[0-9]*)-(?P<snd_max>[0-9]*)").unwrap();
    let ranges = range_re
        .captures_iter(&chunks[0])
        .fold(HashMap::new(), |mut map, cap| {
            map.insert(
                cap["key"].to_string(),
                (
                    parse(&cap["fst_min"]),
                    parse(&cap["fst_max"]),
                    parse(&cap["snd_min"]),
                    parse(&cap["snd_max"]),
                ),
            );
            map
        });
    let own_numbers = chunks[1]
        .split("\n")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    let nearby_numbers = chunks[2]
        .split("\n")
        .skip(1)
        .map(|row| {
            row.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    (ranges, own_numbers, nearby_numbers)
}

fn main() {
    let (ranges, own, nearby) = get_data();
    let mut error_rows = Vec::new();
    let valid_rows: Vec<_> = nearby
        .iter()
        .filter(|row| {
            let err = row.iter().fold(0, |row_tot, n| {
                row_tot
                    + if ranges.iter().fold(
                        false,
                        |valid, (_, (fst_min, fst_max, snd_min, snd_max))| {
                            valid
                                || (fst_min <= n && n <= fst_max)
                                || (snd_min <= n && n <= snd_max)
                        },
                    ) {
                        0
                    } else {
                        *n
                    }
            });
            if err != 0 {
                error_rows.push(err);
                false
            } else {
                true
            }
        })
        .collect();
    println!(
        "First problem: {}",
        error_rows.iter().fold(0, |tot, n| tot + n)
    );

    let mut cols = (0..valid_rows[0].len())
        .map(|_| {
            ranges
                .keys()
                .map(|key| key.clone())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    for row in valid_rows {
        for (i, val) in row.iter().enumerate() {
            cols[i] = cols[i]
                .iter()
                .filter(|&key| {
                    let (fmi, fma, smi, sma) = ranges.get(key).unwrap();
                    (fmi <= val && val <= fma) || (smi <= val && val <= sma)
                })
                .map(|key| key.clone())
                .collect::<Vec<String>>();
        }
    }

    let mut name_index: HashMap<String, usize> = HashMap::new();
    let mut change = true;
    while change {
        change = false;
        for (i, val_col) in cols.clone().iter().enumerate() {
            if val_col.len() == 1 {
                name_index.insert(val_col[0].clone(), i);
                cols = cols
                    .iter()
                    .map(|n| {
                        n.iter()
                            .filter(|&key| *key != val_col[0])
                            .map(|key| key.clone())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>();
                change = true;
                break;
            }
        }
    }
    let snd = name_index
        .iter()
        .filter(|(k, _)| k.contains("departure"))
        .map(|(_, &i)| i)
        .collect::<Vec<usize>>()
        .iter()
        .fold(1, |f, &i| f * own[i]);
    println!("Second problem: {}", snd);
}
