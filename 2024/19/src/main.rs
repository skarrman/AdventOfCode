use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn get_data() -> (HashSet<String>, Vec<String>) {
    let path = "src/input.txt";
    let input = fs::read_to_string(path).expect("Could not read file");
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    (
        parts[0].split(", ").map(|s| String::from(s)).collect(),
        parts[1].split("\n").map(|s| String::from(s)).collect(),
    )
}

fn validate_design(
    design: String,
    patterns: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
    longest_pattern: usize,
) -> usize {
    if cache.contains_key(&design) {
        return cache[&design];
    }
    if design.is_empty() {
        return 1;
    }
    let mut arrangements = 0;
    for i in 1..=(if design.len() < longest_pattern + 1 {
        design.len()
    } else {
        longest_pattern
    }) {
        if patterns.contains(&design[0..i].to_string()) {
            let extra = validate_design(design[i..].to_string(), patterns, cache, longest_pattern);
            if extra == 0 {
                continue;
            }
            arrangements = if arrangements == 0 {
                extra
            } else {
                arrangements + extra
            };
        }
    }

    cache.insert(design, arrangements);
    arrangements
}

fn find_valid_patterns(patterns: HashSet<String>, designs: Vec<String>) -> usize {
    let mut cache = HashMap::new();
    let longest_pattern = patterns
        .iter()
        .fold(0, |max, pat| if max < pat.len() { pat.len() } else { max });
    designs.iter().enumerate().fold(0, |valid, (i, design)| {
        valid
            + if validate_design(design.clone(), &patterns, &mut cache, longest_pattern) > 0 {
                1
            } else {
                0
            }
    })
}

fn find_number_of_arrangements(patterns: HashSet<String>, designs: Vec<String>) -> usize {
    let mut cache = HashMap::new();
    let longest_pattern = patterns
        .iter()
        .fold(0, |max, pat| if max < pat.len() { pat.len() } else { max });
    designs.iter().enumerate().fold(0, |arr, (i, design)| {
        arr + validate_design(design.clone(), &patterns, &mut cache, longest_pattern)
    })
}

fn main() {
    let input = get_data();
    match env::var("part") {
        Ok(part) if part == "part2" => {
            println!("{}", find_number_of_arrangements(input.0, input.1))
        }
        _ => println!("{}", find_valid_patterns(input.0, input.1)),
    }
}
