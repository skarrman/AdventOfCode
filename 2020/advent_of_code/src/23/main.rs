use std::fs;

fn get_data() -> Vec<usize> {
    let path = "src/23/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .chars()
        .map(|ch| format!("{}", ch).parse().unwrap())
        .collect()
}

fn play(mut cur: usize, mut cups: Vec<usize>, rounds: usize) -> Vec<usize> {
    for _ in 0..rounds {
        let fst = cups[cur];
        let snd = cups[fst];
        let trd = cups[snd];

        let mut dest = if cur == 1 { cups.len() - 1 } else { cur - 1 };
        while [fst, snd, trd].contains(&dest) {
            dest = if dest == 1 { cups.len() - 1 } else { dest - 1 };
        }
        cups[cur] = cups[trd];
        let af_next = cups[dest];
        cups[dest] = fst;
        cups[fst] = snd;
        cups[snd] = trd;
        cups[trd] = af_next;
        cur = cups[cur];
    }
    cups
}

fn to_linked(lst: &Vec<usize>) -> Vec<usize> {
    let mut linked = vec![0; lst.len() + 1];
    for win in lst.windows(2) {
        linked[win[0]] = win[1];
    }
    linked[lst[lst.len() - 1]] = lst[0];
    linked
}

fn main() {
    let mut data = get_data();
    let cups = play(data[0], to_linked(&data), 100);
    let mut i = cups[1];
    let mut s = "".to_string();
    while i != 1 {
        s = format!("{}{}", s, i);
        i = cups[i];
    }
    println!("First problem: {}", s);
    data.extend(9 + 1..=1000000);
    let cups = play(data[0], to_linked(&data), 10000000);
    let fst = cups[1];
    let snd = cups[fst];
    println!("Second problem: {}", fst * snd);
}
