use std::collections::HashMap;
use std::env;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Game {
    turn: usize,
    score: Vec<i32>,
    pos: Vec<i32>,
}

fn play_game(game: &Game, lookup: &mut HashMap<Game, (u64, u64)>) -> (u64, u64) {
    if lookup.contains_key(&game) {
        return lookup[&game];
    }
    if game.score[(game.turn + 1) % 2] >= 21 {
        return (
            if game.turn == 0 { 0 } else { 1 },
            if game.turn == 0 { 1 } else { 0 },
        );
    }
    let (mut w1, mut w2) = (0, 0);
    for fst in 1..=3 {
        for snd in 1..=3 {
            for trd in 1..=3 {
                let p = (game.pos[game.turn] + fst + snd + trd) % 10;
                let s = game.score[game.turn] + p + 1;
                let mut ps = game.pos.clone();
                ps[game.turn] = p;
                let mut ss = game.score.clone();
                ss[game.turn] = s;
                let (_w1, _w2) = play_game(
                    &Game {
                        turn: (game.turn + 1) % 2,
                        score: ss,
                        pos: ps,
                    },
                    lookup,
                );
                w1 += _w1;
                w2 += _w2;
            }
        }
    }
    lookup.insert(game.clone(), (w1, w2));
    (w1, w2)
}

fn fst(p1: i32, p2: i32) {
    let mut dice_rolls = 0;
    let mut dice = 1;
    let mut players = vec![(p1, 0), (p2, 0)];
    loop {
        let mut round = vec![];
        let mut win = false;
        for (mut p, s) in &players {
            let mut rolls = 0;
            for _ in 0..3 {
                rolls += dice;
                dice = if (dice + 1) > 100 { 1 } else { dice + 1 };
                dice_rolls += 1;
            }
            p = (p + rolls) % 10;
            if s + p + 1 >= 1000 {
                win = true;
                break;
            }
            round.push((p, s + p + 1));
        }
        if win {
            break;
        }
        players = round;
    }
    println!(
        "{}",
        (players.iter().map(|&(_, s)| s).min().unwrap() * dice_rolls)
    );
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let (p1, p2) = (9, 1); // Decrease pos by 1
    match part.as_ref() {
        "part1" => fst(p1, p2),
        _ => {
            let (w1, w2) = play_game(
                &Game {
                    turn: 0,
                    score: vec![0, 0],
                    pos: vec![p1, p2],
                },
                &mut HashMap::new(),
            );
            println!("{}", if w1 > w2 { w1 } else { w2 })
        }
    }
}
