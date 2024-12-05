use std::{collections::HashMap, env, fs, usize};

fn get_data() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let parts: Vec<&str> = file_contents.split("\n\n").collect();
    let map = parts[0].split("\n").fold(HashMap::new(), |mut m, row| {
        let pair = row.split("|").collect::<Vec<&str>>();
        m.entry(pair[1].parse::<i32>().unwrap())
            .or_insert(vec![])
            .push(pair[0].parse::<i32>().unwrap());
        m
    });
    let insts = parts[1]
        .split("\n")
        .map(|row| row.split(",").map(|i| i.parse::<i32>().unwrap()).collect())
        .collect();
    (map, insts)
}

fn index_of(list: &Vec<i32>, val: i32) -> usize {
    for (i, v) in list.iter().enumerate() {
        if *v == val {
            return i;
        }
    }
    usize::MAX
}

fn all_before(list: &Vec<i32>, vals: Vec<i32>, i: usize) -> (bool, usize) {
    for val in vals {
        let index = index_of(list, val);
        if index != usize::MAX && index >= i {
            return (false, index);
        }
    }
    (true, usize::MAX)
}

fn verify_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    update.iter().enumerate().fold(true, |p, (i, up)| {
        p && (!rules.contains_key(up) || all_before(update, rules[up].clone(), i).0)
    })
}

fn fix_update(rules: &HashMap<i32, Vec<i32>>, update_input: &Vec<i32>) -> i32 {
    let mut edited = true;
    let mut update = update_input.clone();
    while edited {
        edited = false;
        for (i, k) in update.clone().iter().enumerate() {
            if !rules.contains_key(k) {
                continue;
            }
            let check = all_before(&update, rules[k].clone(), i);
            if !check.0 {
                let mut new_update = update.clone();
                let tmp = new_update[i];
                new_update[i] = new_update[check.1];
                new_update[check.1] = tmp;
                edited = true;
                update = new_update
            }
        }
    }

    update[update.len() / 2]
}

fn main() {
    let (rules, updates) = get_data();

    match env::var("part") {
        Ok(part) if part == "part2" => {
            println!(
                "{}",
                updates
                    .iter()
                    .filter(|u| !verify_update(&rules, &u))
                    .fold(0, |sum, update| sum + fix_update(&rules, &update))
            )
        }
        _ => println!(
            "{}",
            updates.into_iter().fold(0, |sum, update| {
                sum + if verify_update(&rules, &update) {
                    update[update.len() / 2]
                } else {
                    0
                }
            })
        ),
    }
}
