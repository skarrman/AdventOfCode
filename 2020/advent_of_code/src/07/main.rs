use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
extern crate regex;

fn get_data() -> Vec<String> {
    let path = "src/07/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|group| String::from(group.replace("bags", "").replace("bag", "").to_lowercase()))
        .collect()
}

fn num_name(key: &str) -> (i32, String) {
    let re_num = regex::Regex::new(r"\s(?P<num>[0-9]*)\s(?P<bag>[a-z ]*)").unwrap();
    let cap = re_num.captures_iter(key).next().unwrap();
    let num = cap["num"].parse::<i32>().unwrap();
    let bag = String::from(&cap["bag"]).replace(" ", "");
    (num, bag)
}

fn search(bag_map: &HashMap<String, Vec<(i32, String)>>, bag: &String) -> i32 {
    match bag_map.get(bag) {
        Some(vec) => vec.iter().fold(0, |bags, (num, next_bag)| {
            bags + num + num * search(bag_map, next_bag)
        }),
        None => 0,
    }
}

fn main() {
    let mut can_be_in: HashMap<String, Vec<String>> = HashMap::new();
    let mut require_to_have: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for row in get_data() {
        let bags: Vec<&str> = row.split("contain").collect();
        let val = String::from(bags[0]).replace(" ", "");
        if !bags[1].contains("no other") {
            for key in bags[1].split(",") {
                let (num, bag) = num_name(key);
                let vec = can_be_in.entry(bag.clone()).or_insert(Vec::new());
                vec.push(val.clone());
                let vec = require_to_have.entry(val.clone()).or_insert(Vec::new());
                vec.push((num, bag));
            }
        }
    }
    let mut keys: VecDeque<String> = VecDeque::new();
    keys.push_back(String::from("shinygold"));
    let mut bags: HashSet<String> = HashSet::new();
    while keys.len() > 0 {
        let key = keys.pop_front().unwrap();
        match can_be_in.get(&key) {
            Some(next_keys) => {
                for next_key in next_keys {
                    bags.insert(next_key.clone());
                    keys.push_back(next_key.clone());
                }
            }
            None => (),
        }
    }
    println!("First challenge: {}", bags.len());
    let bags = search(&require_to_have, &String::from("shinygold"));
    println!("Second challenge: {}", bags);
}
