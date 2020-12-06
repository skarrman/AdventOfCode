use std::collections::HashSet;
use std::fs;

fn get_data() -> Vec<String> {
    let path = "src/06/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n\n")
        .map(|group| String::from(group))
        .collect()
}

fn main() {
    let data = get_data();
    let num_yes = data
        .iter()
        .map(|group| {
            let mut anss = HashSet::new();
            for ans in group.replace("\n", "").chars() {
                anss.insert(ans);
            }
            anss
        })
        .fold(0, |num_ans, ans| num_ans + ans.len());
    println!("First challenge: {}", num_yes);
    let num_same = data
        .iter()
        .map(|group| {
            let ans: Vec<HashSet<char>> = group
                .split("\n")
                .map(|per| {
                    per.chars().fold(HashSet::new(), |mut ans, c| {
                        ans.insert(c);
                        ans
                    })
                })
                .collect();
            let mut comb = ans[0].clone();
            for an in ans {
                let mut tmp = HashSet::new();
                for c in comb.intersection(&an) {
                    tmp.insert(*c);
                }
                comb = tmp;
            }
            comb
        })
        .fold(0, |tot, ans| tot + ans.len());
    println!("Second challenge: {}", num_same);
}
