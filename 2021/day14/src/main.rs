use std::collections::HashMap;
use std::env;
use std::fs;

fn get_data() -> (
    HashMap<(char, char), u64>,
    HashMap<(char, char), char>,
    char,
) {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not read file");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let template =
        parts[0]
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .fold(HashMap::new(), |mut map, pair| {
                *map.entry((pair[0], pair[1])).or_insert(0) += 1;
                map
            });
    let mapping = parts[1]
        .split("\n")
        .map(|l| {
            let ps = l.split(" -> ").collect::<Vec<&str>>();
            let mut key = ps[0].chars();
            (
                (key.next().unwrap(), key.next().unwrap()),
                ps[1].chars().next().unwrap(),
            )
        })
        .collect();
    (template, mapping, parts[0].chars().next().unwrap())
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let (mut chars, mapping, fst) = get_data();
    for _ in 0..(if part == "part2" { 40 } else { 10 }) {
        let mut new: HashMap<(char, char), u64> = HashMap::new();
        for ((c1, c2), o) in &chars {
            *new.entry((*c1, mapping[&(*c1, *c2)])).or_insert(0) += o;
            *new.entry((mapping[&(*c1, *c2)], *c2)).or_insert(0) += o;
        }
        chars = new;
    }
    let (min, max) = chars
        .iter()
        .fold(HashMap::from([(fst, 1)]), |mut map, ((_, c2), o)| {
            *map.entry(*c2).or_insert(0) += *o;
            map
        })
        .iter()
        .fold((0xFFFFFFFFFFFFFFFF, 0), |(mi, ma), (_, o)| {
            (if *o < mi { *o } else { mi }, if *o > ma { *o } else { ma })
        });
    println!("{}", max - min);
}
