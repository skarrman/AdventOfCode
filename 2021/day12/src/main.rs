use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn not_contains_lowercase_dup(path: &LinkedList<String>) -> bool {
    let lowercase: Vec<&String> = path
        .iter()
        .filter(|s| s.chars().next().unwrap() > 'Z')
        .collect();
    let unique: HashSet<_> = lowercase.iter().map(|s| s).collect();
    unique.len() == lowercase.len()
}

fn dfs(
    current: &String,
    paths: &HashMap<String, HashSet<String>>,
    mut path: LinkedList<String>,
    condition: fn(&String, &LinkedList<String>) -> bool,
) -> Vec<LinkedList<String>> {
    path.push_back(current.clone());
    if *current == "end".to_string() {
        return vec![path.clone()];
    }
    let mut paths_to_end: Vec<LinkedList<String>> = vec![];
    for next in paths.get(current).unwrap() {
        if next.chars().next().unwrap() < 'a' || condition(next, &path) {
            match dfs(next, paths, path.clone(), condition) {
                ps if ps.len() > 0 => {
                    for p in ps {
                        paths_to_end.push(p);
                    }
                }
                _ => continue,
            }
        }
    }
    paths_to_end
}

fn read_input_as_integers(path: &str) -> HashMap<String, HashSet<String>> {
    let file = File::open(path).expect("not found");
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let parts = l.split("-").collect::<Vec<&str>>();
        let (from, to) = (parts[0].to_string(), parts[1].to_string());
        map.entry(from.clone()).or_insert(HashSet::new());
        map.get_mut(&from).unwrap().insert(to.clone());
        map.entry(to.clone()).or_insert(HashSet::new());
        map.get_mut(&to).unwrap().insert(from);
    }
    map
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    println!(
        "{}",
        dfs(
            &"start".to_string(),
            &read_input_as_integers("input.txt"),
            LinkedList::new(),
            if part == "part1" {
                |next, path| !path.contains(next)
            } else {
                |next, path| {
                    *next != "start".to_string()
                        && (!path.contains(next) || not_contains_lowercase_dup(&path))
                }
            },
        )
        .len()
    );
}
