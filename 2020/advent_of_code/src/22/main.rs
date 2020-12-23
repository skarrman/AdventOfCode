use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn deck_of(str_data: Option<&str>) -> VecDeque<usize> {
    str_data
        .unwrap()
        .split("\n")
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect()
}
fn get_data() -> (VecDeque<usize>, VecDeque<usize>) {
    let path = "src/22/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let mut player_split = file_contents.split("\n\n");
    (deck_of(player_split.next()), deck_of(player_split.next()))
}

fn combat(mut d1: VecDeque<usize>, mut d2: VecDeque<usize>) -> usize {
    let winning = loop {
        match (d1.len(), d2.len()) {
            (0, _) => break d2,
            (_, 0) => break d1,
            _ => match (d1.pop_front().unwrap(), d2.pop_front().unwrap()) {
                (c1, c2) if c1 > c2 => {
                    d1.push_back(c1);
                    d1.push_back(c2);
                }
                (c1, c2) => {
                    d2.push_back(c2);
                    d2.push_back(c1);
                }
                _ => panic!("Error in round"),
            },
        }
    };

    winning
        .iter()
        .rev()
        .zip(1..=winning.len())
        .fold(0, |res, (c, o)| res + c * o)
}

fn recursive_combat(d1: VecDeque<usize>, d2: VecDeque<usize>) -> usize {
    let (w1, d1, d2) = rec_comb(d1, d2);
    let (d, l) = if w1 {
        (d1.clone(), d1.len())
    } else {
        (d2.clone(), d2.len())
    };
    d.iter().rev().zip(1..=l).fold(0, |res, (c, o)| res + c * o)
}

fn rec_comb(
    mut d1: VecDeque<usize>,
    mut d2: VecDeque<usize>,
) -> (bool, VecDeque<usize>, VecDeque<usize>) {
    let mut hist: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();
    loop {
        if hist.contains(&(d1.clone(), d2.clone())) {
            break (true, d1, d2);
        }
        match (d1.len(), d2.len()) {
            (0, _) => break (false, d1, d2),
            (_, 0) => break (true, d1, d2),
            _ => {
                hist.insert((d1.clone(), d2.clone()));
                match (d1.pop_front().unwrap(), d2.pop_front().unwrap()) {
                    (c1, c2) if d1.len() >= c1 && d2.len() >= c2 => {
                        let (mut _d1, mut _d2) = (d1.clone(), d2.clone());
                        _d1.truncate(c1);
                        _d2.truncate(c2);
                        match rec_comb(_d1, _d2) {
                            (true, _, _) => {
                                d1.push_back(c1);
                                d1.push_back(c2);
                            }
                            _ => {
                                d2.push_back(c2);
                                d2.push_back(c1);
                            }
                        }
                    }
                    (c1, c2) if c1 > c2 => {
                        d1.push_back(c1);
                        d1.push_back(c2);
                    }
                    (c1, c2) => {
                        d2.push_back(c2);
                        d2.push_back(c1);
                    }
                }
            }
        }
    }
}

fn main() {
    let (d1, d2) = get_data();
    println!("First problem: {}", combat(d1.clone(), d2.clone()));
    println!("Second problem: {}", recursive_combat(d1, d2));
}
