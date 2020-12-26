use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
enum Rule {
    Let(char),
    Ref(Vec<usize>),
    Or(Box<Rule>, Box<Rule>),
}

fn get_data() -> (HashMap<usize, Rule>, Vec<String>) {
    let path = "src/19/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    (
        file_contents
            .split("\n\n")
            .next()
            .unwrap()
            .split("\n")
            .fold(HashMap::new(), |mut map, row| {
                let (is_let, is_or) = (row.contains("\""), row.contains("|"));
                let mut key_split = row.split(": ");
                let key = key_split.next().unwrap().parse().unwrap();
                let rules = key_split.collect::<String>();
                if is_let {
                    map.insert(key, Rule::Let(rules.replace("\"", "").parse().unwrap()));
                } else if is_or {
                    let conds = rules
                        .split(" | ")
                        .map(|sub| sub.split(" ").map(|r| r.parse().unwrap()).collect())
                        .collect::<Vec<Vec<usize>>>();
                    map.insert(
                        key,
                        Rule::Or(
                            Box::new(Rule::Ref(conds[0].clone())),
                            Box::new(Rule::Ref(conds[1].clone())),
                        ),
                    );
                } else {
                    map.insert(
                        key,
                        Rule::Ref(
                            rules
                                .split(" ")
                                .map(|r| r.parse().unwrap())
                                .collect::<Vec<usize>>(),
                        ),
                    );
                }
                map
            }),
        file_contents
            .split("\n\n")
            .last()
            .unwrap()
            .split("\n")
            .map(|row| row.to_string())
            .collect(),
    )
}

fn valid(
    rules: &HashMap<usize, Rule>,
    rule: &Rule,
    input: String,
    depth: usize,
) -> (bool, Vec<String>) {
    if depth >= 1000 {
        (false, Vec::new())
    } else {
        match rule {
            Rule::Let(l) => {
                if input.len() > 0 {
                    (
                        input.chars().next().unwrap() == *l,
                        vec![input.chars().skip(1).collect()],
                    )
                } else {
                    (false, vec![])
                }
            }
            Rule::Ref(refs) => {
                let mut inputs = vec![input];
                for rule in refs {
                    let mut _inputs = Vec::new();
                    for input in &inputs {
                        match valid(
                            rules,
                            rules.get(rule).unwrap(),
                            input.to_string(),
                            depth + 1,
                        ) {
                            (true, _input) => {
                                _inputs.extend(_input);
                            }
                            _ => (),
                        };
                    }
                    if _inputs.len() == 0 {
                        return (false, Vec::new());
                    }
                    inputs = _inputs;
                }
                (true, inputs)
            }
            Rule::Or(r1, r2) => {
                let (v1, s1) = valid(rules, r1, input.clone(), depth + 1);
                let (v2, s2) = valid(rules, r2, input, depth + 1);
                let mut ss = Vec::new();
                if v1 {
                    ss.extend(s1);
                }
                if v2 {
                    ss.extend(s2);
                }
                (v1 || v2, ss)
            }
        }
    }
}

fn check_valid(rules: &HashMap<usize, Rule>, input: String) -> bool {
    match valid(rules, rules.get(&0).unwrap(), input.clone(), 0) {
        (true, ss) if ss.iter().fold(false, |res, s| res || s.len() == 0) => true,
        _ => false,
    }
}

fn main() {
    let (mut rules, inputs) = get_data();
    let matched = inputs.iter().fold(0, |matched, input| {
        matched
            + if check_valid(&rules, input.to_string()) {
                1
            } else {
                0
            }
    });
    println!("First problem: {}", matched);
    rules.insert(
        8,
        Rule::Or(
            Box::new(Rule::Ref(vec![42])),
            Box::new(Rule::Ref(vec![42, 8])),
        ),
    ); // 8: 42 | 42 8
    rules.insert(
        11,
        Rule::Or(
            Box::new(Rule::Ref(vec![42, 31])),
            Box::new(Rule::Ref(vec![42, 11, 31])),
        ),
    ); // 11: 42 31 | 42 11 31
    let matched = inputs.iter().fold(0, |matched, input| {
        matched
            + if check_valid(&rules, input.to_string()) {
                1
            } else {
                0
            }
    });
    println!("Second problem: {}", matched);
}
