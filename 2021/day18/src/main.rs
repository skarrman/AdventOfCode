use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Clone)]
enum Elem {
    Num(i32),
    Pair(Box<Elem>, Box<Elem>),
}

#[derive(Clone)]
enum Token {
    E(Elem),
    Op,
    Cl,
}

fn to_str(e: Elem) -> String {
    match e {
        Elem::Num(n) => format!("{}", n),
        Elem::Pair(l, r) => format!("[{},{}]", to_str(*l), to_str(*r)),
    }
}

fn print_elem(e: Elem) {
    println!("{}", to_str(e));
}

fn tokenize(line: &str) -> VecDeque<Token> {
    line.replace("[", "_[_")
        .replace("]", "_]_")
        .replace(",", "_")
        .split("_")
        .filter(|s| *s != "")
        .map(|c| match c {
            "[" => Token::Op,
            "]" => Token::Cl,
            n => Token::E(Elem::Num(n.parse::<i32>().unwrap())),
        })
        .collect()
}

fn parse(mut tokens: VecDeque<Token>) -> Elem {
    let mut stack: VecDeque<Token> = VecDeque::new();
    while tokens.len() > 0 || stack.len() > 1 {
        if stack.len() >= 4 {
            let (fth, trd, snd, fst) = (
                stack[0].clone(),
                stack[1].clone(),
                stack[2].clone(),
                stack[3].clone(),
            );

            match (fst, snd, trd, fth) {
                (Token::Op, Token::E(l), Token::E(r), Token::Cl) => {
                    stack = stack.split_off(4);
                    stack.push_front(Token::E(Elem::Pair(Box::new(l), Box::new(r))))
                }
                _ => stack.push_front(tokens.pop_front().unwrap()),
            }
        } else {
            stack.push_front(tokens.pop_front().unwrap());
        }
    }
    match stack[0].clone() {
        Token::E(elem) => elem,
        _ => panic!("Parsing did not result in single element"),
    }
}

fn add(lhs: &Elem, rhs: &Elem) -> Elem {
    Elem::Pair(Box::new(lhs.clone()), Box::new(rhs.clone()))
}

fn explode(elem: &Elem, level: i32) -> (Option<Elem>, (Option<i32>, Option<i32>)) {
    match (elem.clone(), level) {
        (Elem::Pair(l, r), 4) => match (*l, *r) {
            (Elem::Num(ln), Elem::Num(rn)) => (None, (Some(ln), Some(rn))),
            _ => panic!(),
        },
        (Elem::Pair(l, r), n) => match explode(&l, n + 1) {
            (None, (Some(ln), Some(rn))) => match *r {
                Elem::Num(n) => (
                    Some(Elem::Pair(
                        Box::new(Elem::Num(0)),
                        Box::new(Elem::Num(rn + n)),
                    )),
                    (Some(ln), None),
                ),
                Elem::Pair(l1, r1) => (
                    Some(Elem::Pair(
                        Box::new(Elem::Num(0)),
                        Box::new(Elem::Pair(Box::new(put_left(&*l1, rn)), r1)),
                    )),
                    (Some(ln), None),
                ),
                _ => panic!(),
            },
            (Some(el), (le, Some(rn))) => match *r {
                Elem::Num(re) => (
                    Some(Elem::Pair(Box::new(el), Box::new(Elem::Num(rn + re)))),
                    (le, None),
                ),
                _ => (
                    Some(Elem::Pair(Box::new(el), Box::new(put_left(&*r, rn)))),
                    (le, None),
                ),
            },
            (Some(el), (ln, rn)) => (Some(Elem::Pair(Box::new(el), r)), (ln, rn)),
            (None, (None, None)) => match explode(&r, n + 1) {
                (None, (Some(ln), Some(rn))) => match *l {
                    Elem::Num(n) => (
                        Some(Elem::Pair(
                            Box::new(Elem::Num(ln + n)),
                            Box::new(Elem::Num(0)),
                        )),
                        (None, Some(rn)),
                    ),
                    _ => panic!(),
                },
                (Some(el), (Some(ln), re)) => match *l {
                    Elem::Num(le) => (
                        Some(Elem::Pair(Box::new(Elem::Num(ln + le)), Box::new(el))),
                        (None, re),
                    ),
                    _ => (
                        Some(Elem::Pair(Box::new(put_right(&*l, ln)), Box::new(el))),
                        (None, re),
                    ),
                },
                (Some(el), (ln, rn)) => (Some(Elem::Pair(l, Box::new(el))), (ln, rn)),
                (None, ns) => (None, ns),
            },
            _ => panic!(),
        },
        _ => (None, (None, None)),
    }
}

fn put_right(elem: &Elem, num: i32) -> Elem {
    match elem {
        Elem::Num(n) => Elem::Num(num + n),
        Elem::Pair(l, r) => Elem::Pair(l.clone(), Box::new(put_right(r, num))),
    }
}

fn put_left(elem: &Elem, num: i32) -> Elem {
    match elem {
        Elem::Num(n) => Elem::Num(num + n),
        Elem::Pair(l, r) => Elem::Pair(Box::new(put_left(l, num)), r.clone()),
    }
}

fn split(elem: &Elem) -> Option<Elem> {
    match elem {
        Elem::Num(n) if *n > 9 => Some(Elem::Pair(
            Box::new(Elem::Num(n / 2)),
            Box::new(Elem::Num(if n % 2 == 0 { n / 2 } else { n / 2 + 1 })),
        )),
        Elem::Pair(l, r) => match split(l) {
            Some(l1) => Some(Elem::Pair(Box::new(l1), r.clone())),
            _ => match split(r) {
                Some(r1) => Some(Elem::Pair(l.clone(), Box::new(r1))),
                _ => None,
            },
        },
        _ => None,
    }
}

fn reduce(el: &Elem) -> Elem {
    let mut elem = el.clone();
    loop {
        match explode(&elem, 0) {
            (Some(e), _) => elem = e,
            _ => match split(&elem) {
                Some(e) => elem = e,
                _ => break elem.clone(),
            },
        };
    }
}

fn add_reduce(lhs: &Elem, rhs: &Elem) -> Elem {
    reduce(&add(lhs, rhs))
}

fn sum(elems: Vec<Elem>) -> Elem {
    elems
        .iter()
        .skip(1)
        .fold(elems[0].clone(), |e1, e2| add_reduce(&e1, &e2))
}

fn magnitude(elem: Elem) -> i32 {
    match elem {
        Elem::Num(n) => n,
        Elem::Pair(l, r) => 3 * magnitude(*l) + 2 * magnitude(*r),
    }
}

fn get_data() -> Vec<Elem> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    contents
        .split("\n")
        .map(|line| tokenize(line))
        .map(|tokens| parse(tokens))
        .collect()
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let lines = get_data();
    if part == "part1" {
        println!("{}", magnitude(sum(lines)));
    } else {
        let mut max = 0;
        for i in 0..lines.len() {
            for j in 0..lines.len() {
                if i == j {
                    continue;
                }
                let mag = magnitude(add_reduce(&lines[i], &lines[j]));
                if max < mag {
                    max = mag;
                }
            }
        }
        println!("{}", max);
    }
}
