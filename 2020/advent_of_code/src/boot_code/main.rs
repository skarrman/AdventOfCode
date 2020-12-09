use std::collections::HashSet;
use std::env;
use std::fs;
extern crate regex;

#[derive(Clone)]
enum Code {
    Op(Op),
    Br(Br),
}

#[derive(Clone)]
enum Br {
    Jmp(i32),
}

#[derive(Clone)]
enum Op {
    Nop(i32),
    Acc(i32),
}

struct State {
    pc: usize,
    acc: i32,
}

fn read_code_file(file: String) -> Vec<Code> {
    let file_contents = fs::read_to_string(file).expect("Could not read file");
    file_contents
        .split("\n")
        .fold(Vec::new(), |mut prg, code_str| {
            let split: Vec<&str> = code_str.split(" ").collect();
            let arg = split[1].parse::<i32>().unwrap();
            prg.push(match split[0] {
                "nop" => Code::Op(Op::Nop(arg)),
                "acc" => Code::Op(Op::Acc(arg)),
                "jmp" => Code::Br(Br::Jmp(arg)),
                _ => panic!("Code not recognized!"),
            });
            prg
        })
}

fn eval_code(code: &Code, mut state: State) -> State {
    match &code {
        Code::Op(op) => {
            match op {
                Op::Nop(_) => (),
                Op::Acc(n) => state.acc += n,
            };
            state.pc += 1
        }
        Code::Br(br) => match br {
            Br::Jmp(d) if *d < 0 => state.pc -= (-*d) as usize,
            Br::Jmp(d) => state.pc += *d as usize,
        },
    };
    state
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let path = format!("src/boot_code/{}.txt", day);
    let prg = read_code_file(path);
    let (res1, res2) = match day {
        8 => (run8_1(&prg), run8_2(&prg)),
        _ => panic!("Day not exist"),
    };
    println!(
        "Day {}:\n\tFirst problem: {}\n\tSecond problem: {}",
        day, res1, res2
    );
}

// Specific day problems:

fn run8_1(prg: &Vec<Code>) -> i32 {
    let (mut state, mut hist): (State, HashSet<usize>) = (State { pc: 0, acc: 0 }, HashSet::new());
    while !hist.contains(&state.pc) {
        hist.insert(state.pc);
        state = eval_code(&prg[state.pc], state);
    }
    state.acc
}

fn is_nop_or_jmp(prg: &Vec<Code>, i: usize, is_nop: bool) -> bool {
    match &prg[i] {
        Code::Op(op) => match op {
            Op::Nop(_) => is_nop,
            _ => false,
        },
        Code::Br(_) => !is_nop,
    }
}

fn run8_2(prg: &Vec<Code>) -> i32 {
    let mut nop = true;
    let mut i = 0;
    loop {
        while !is_nop_or_jmp(prg, i, nop) {
            i += 1;
            if i == prg.len() {
                nop = false;
                i = 0;
            }
        }
        let (mut state, mut hist): (State, HashSet<usize>) =
            (State { pc: 0, acc: 0 }, HashSet::new());
        while !hist.contains(&state.pc) && state.pc != prg.len() {
            hist.insert(state.pc);
            let next_code = if state.pc != i {
                prg[state.pc].clone()
            } else {
                match &prg[state.pc] {
                    Code::Op(Op::Nop(d)) => Code::Br(Br::Jmp(*d)),
                    Code::Br(_) => Code::Op(Op::Nop(1)),
                    _ => panic!(""),
                }
            };
            state = eval_code(&next_code, state);
        }
        if state.pc == prg.len() {
            break state.acc;
        }
        i += 1;
    }
}
