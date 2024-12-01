use std::{env, fs};

fn get_data() -> (Vec<i32>, Vec<i32>) {
    let path = "src/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    file_contents
        .split("\n")
        .fold((Vec::new(), Vec::new()), |(mut fst, mut snd), row| {
            let pair = row
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            fst.push(pair[0].parse::<i32>().unwrap());
            snd.push(pair[1].parse::<i32>().unwrap());
            (fst, snd)
        })
}
fn main() {
    let (mut first_list, mut second_list) = get_data();
    first_list.sort();
    second_list.sort();

    match env::var("part") {
        Ok(part) if part == "part2" => println!(
            "{}",
            first_list.iter().fold(0, |score, item| {
                score
                    + item
                        * second_list
                            .iter()
                            .fold(0, |o, n| if item == n { o + 1 } else { o })
            })
        ),
        _ => println!(
            "{}",
            first_list
                .iter()
                .zip(second_list.clone())
                .fold(0, |dists, (a, b)| dists + (a - b).abs())
        ),
    }
}
