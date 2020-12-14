use std::collections::HashMap;
use std::fs;

enum Inst {
    Mask(String, u64, u64),
    Write(u64, u64),
}

fn get_data() -> Vec<Inst> {
    let path = "src/14/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| {
            let row = row.replace(" ", "");
            let parts = row.split("=").collect::<Vec<&str>>();
            match parts[0] {
                "mask" => {
                    let or_mask =
                        u64::from_str_radix(&parts[1].clone().replace("X", "0"), 2).unwrap();
                    let and_mask =
                        u64::from_str_radix(&parts[1].clone().replace("X", "1"), 2).unwrap();
                    Inst::Mask(parts[1].to_string(), or_mask, and_mask)
                }
                _ => {
                    let addr = parts[0]
                        .replace("m", "")
                        .replace("e", "")
                        .replace("[", "")
                        .replace("]", "")
                        .parse::<u64>()
                        .unwrap();
                    let num = parts[1].parse::<u64>().unwrap();
                    Inst::Write(addr, num)
                }
            }
        })
        .collect()
}

fn get_x_positions(mask: &String) -> Vec<usize> {
    mask.chars()
        .enumerate()
        .map(|(i, c)| {
            if c == 'X' {
                {
                    i
                }
            } else {
                64
            }
        })
        .filter(|&i| i != 64)
        .collect()
}

fn one_bits(len: usize) -> i64 {
    (0..len).fold(0, |num, _| (num << 1) + 1)
}

fn main() {
    let data = get_data();
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let (mut or_mask, mut and_mask) = (0, 0);
    for inst in &data {
        match inst {
            Inst::Mask(_, or, and) => {
                or_mask = *or;
                and_mask = *and;
            }
            Inst::Write(addr, mut num) => {
                num &= and_mask;
                num |= or_mask;
                mem.insert(*addr, num);
            }
        }
    }
    let sum = mem.iter().fold(0, |s, (_, v)| s + v);
    println!("First problem: {}", sum);
    mem.clear();
    let mut mask = "".to_string();
    for inst in data {
        match inst {
            Inst::Mask(m, or, _) => {
                mask = m;
                or_mask = or;
            }
            Inst::Write(mut addr, num) => {
                addr |= or_mask;
                let is = get_x_positions(&mask);
                let mut comb = one_bits(is.len());
                while comb >= 0 {
                    let mut addr_mask = addr;
                    for (b, i) in is.iter().enumerate() {
                        if (1 << b) & comb > 0 {
                            addr_mask |= 1u64.rotate_right(((64 - mask.len()) + *i + 1usize) as u32)
                        } else {
                            addr_mask &= (0..64).fold(0, |n, _i| {
                                (n << 1) + if 64 - mask.len() + *i == _i { 0 } else { 1 }
                            });
                        }
                    }
                    mem.insert(addr_mask, num);
                    comb -= 1;
                }
            }
        }
    }
    let sum = mem.iter().fold(0, |s, (_, v)| s + v);
    println!("Second problem: {}", sum);
}
