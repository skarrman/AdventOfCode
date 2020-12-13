use std::fs;

fn get_data() -> (i128, Vec<i128>) {
    let path = "src/13/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    let mut split = file_contents.split("\n");
    let ts = split.next().unwrap().parse::<i128>().unwrap();
    (
        ts,
        split
            .next()
            .unwrap()
            .split(",")
            .map(|n| {
                if n == "x" {
                    0
                } else {
                    n.parse::<i128>().unwrap()
                }
            })
            .collect(),
    )
}

fn eea(x: i128, y: i128) -> (i128, i128, i128) {
    match (x, y) {
        (0, b) => (b, 0, 1),
        (a, b) => {
            let q = b / a;
            let r = b % a;
            let (g, s, t) = eea(r, a);
            (g, t - q * s, s)
        }
    }
}

fn main() {
    let (ts, ids) = get_data();
    let (id, t): (i128, i128) = ids
        .iter()
        .filter(|&id| *id != 0)
        .map(|&id| (id, ((ts / id) * id + id) - ts))
        .collect::<Vec<(i128, i128)>>()
        .iter()
        .fold(
            (0, ts),
            |(id, t), &(_id, _t)| {
                if _t < t {
                    (_id, _t)
                } else {
                    (id, t)
                }
            },
        );

    println!("First problem: {}", id * t);

    let mut id_diffs: Vec<(i128, i128)> = Vec::new();
    let mut d = 0;
    for id in ids {
        if id != 0 {
            id_diffs.push((id, d))
        }
        d += 1;
    }

    let mut id_diffs_it = id_diffs.iter();
    let (mut n, mut x) = id_diffs_it.next().unwrap();
    for &(_n, a) in id_diffs_it {
        let (_, m, _m) = eea(n, _n);
        x = x * _m * _n + (_n - a) * m * n;
        n *= _n;
        x = ((x % n) + n) % n
    }
    println!("Second problem: {}", x);
}
