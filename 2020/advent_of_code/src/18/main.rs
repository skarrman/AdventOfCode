use std::collections::VecDeque;
use std::fs;

#[derive(Clone, PartialEq)]
enum Token {
    Num(i64),
    OpSign(Op),
    OpPar,
    ClPar,
}

fn tokenize(line: String) -> VecDeque<Token> {
    line.replace(" ", "")
        .replace("+", "_+_")
        .replace("*", "_*_")
        .replace("(", "_(_")
        .replace(")", "_)_")
        .split("_")
        .filter(|&token| token != "")
        .map(|token| match token {
            "+" => Token::OpSign(Op::Add),
            "*" => Token::OpSign(Op::Mul),
            "(" => Token::OpPar,
            ")" => Token::ClPar,
            _ => Token::Num(token.parse::<i64>().unwrap()),
        })
        .collect()
}
#[derive(Clone, PartialEq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone)]
enum Exp {
    Num(i64),
    Operation(Box<Exp>, Op, Box<Exp>),
}

#[derive(Clone)]
enum P {
    T(Token),
    E(Exp),
}

fn parse(mut tokens: VecDeque<Token>) -> Exp {
    let mut stack: VecDeque<P> = VecDeque::new();
    while tokens.len() != 0 || stack.len() > 1 {
        if stack.len() >= 1 {
            match stack[0] {
                P::T(Token::Num(n)) => {
                    stack.pop_front();
                    stack.push_front(P::E(Exp::Num(n)))
                }
                _ => (),
            }
        }
        if stack.len() >= 3 {
            let (trd, snd, fst) = (stack[0].clone(), stack[1].clone(), stack[2].clone());
            match (fst, snd, trd) {
                (P::T(Token::OpPar), P::E(e), P::T(Token::ClPar)) => {
                    stack = stack.split_off(3);
                    stack.push_front(P::E(e));
                    continue;
                }
                (P::E(e1), P::T(Token::OpSign(op)), P::E(e2)) => {
                    stack = stack.split_off(3);
                    stack.push_front(P::E(Exp::Operation(Box::new(e1), op, Box::new(e2))));
                    continue;
                }
                _ => (),
            }
        }
        stack.push_front(P::T(tokens.pop_front().unwrap()));
    }
    if let P::E(exp) = stack[0].clone() {
        exp
    } else {
        panic!("Parse error")
    }
}

fn parseAdd(mut tokens: VecDeque<Token>) -> Exp {
    let mut stack: VecDeque<P> = VecDeque::new();
    while tokens.len() != 0 || stack.len() > 1 {
        if stack.len() >= 1 {
            match stack[0] {
                P::T(Token::Num(n)) => {
                    stack.pop_front();
                    stack.push_front(P::E(Exp::Num(n)))
                }
                _ => (),
            }
        }
        if stack.len() >= 3 {
            let (trd, snd, fst) = (stack[0].clone(), stack[1].clone(), stack[2].clone());
            match (fst, snd, trd) {
                (P::T(Token::OpPar), P::E(e), P::T(Token::ClPar)) => {
                    stack = stack.split_off(3);
                    stack.push_front(P::E(e));
                    continue;
                }
                (P::E(e1), P::T(Token::OpSign(Op::Add)), P::E(e2)) => {
                    stack = stack.split_off(3);
                    stack.push_front(P::E(Exp::Operation(Box::new(e1), Op::Add, Box::new(e2))));
                    continue;
                }
                (P::E(e1), P::T(Token::OpSign(op)), P::E(e2)) => {
                    if tokens.len() == 0 || tokens[0] != Token::OpSign(Op::Add) {
                        stack = stack.split_off(3);
                        stack.push_front(P::E(Exp::Operation(Box::new(e1), op, Box::new(e2))));
                        continue;
                    }
                }
                _ => (),
            }
        }
        stack.push_front(P::T(tokens.pop_front().unwrap()));
    }
    if let P::E(exp) = stack[0].clone() {
        exp
    } else {
        panic!("Parse error")
    }
}

fn get_data() -> Vec<String> {
    let path = "src/18/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| row.to_string())
        .collect()
}

fn eval(exp: Exp) -> i64 {
    match exp {
        Exp::Num(n) => n,
        Exp::Operation(e1, op, e2) => {
            let n1 = eval(*e1);
            let n2 = eval(*e2);
            match op {
                Op::Add => n1 + n2,
                Op::Mul => n1 * n2,
            }
        }
    }
}

fn main() {
    let data = get_data();
    let sum = data
        .iter()
        .map(|line| tokenize(line.to_string()))
        .map(|tokens| parse(tokens))
        .fold(0, |sum, exp| sum + eval(exp.clone()));
    println!("First problem: {}", sum);
    let sum_snd = data
        .iter()
        .map(|line| tokenize(line.to_string()))
        .map(|tokens| parseAdd(tokens))
        .fold(0, |sum, exp| sum + eval(exp.clone()));
    println!("Second problem: {}", sum_snd);
}
