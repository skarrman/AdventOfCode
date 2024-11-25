use std::collections::HashMap;
use std::collections::LinkedList;
use std::env;
use std::fs;

fn get_data() -> String {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not read file");
    contents.replace("\n", "t")
}

fn pop(iter: &mut LinkedList<char>, num: i64) -> String {
    let mut builder = vec![];
    for _ in 0..num {
        builder.push(iter.pop_front().unwrap());
    }
    builder.iter().collect()
}

fn parse_package(bin_iter: &mut LinkedList<char>) -> (i64, i64) {
    let mut ver = isize::from_str_radix(&pop(bin_iter, 3), 2)
        .unwrap()
        .try_into()
        .unwrap();
    let num: i64;
    match isize::from_str_radix(&&pop(bin_iter, 3), 2).unwrap() {
        4 => {
            let mut builder = "".to_string();
            loop {
                let group = pop(bin_iter, 5);
                match group.chars().next().unwrap() {
                    '1' => {
                        builder =
                            format!("{}{}", builder, group.chars().skip(1).collect::<String>())
                    }
                    _ => {
                        builder =
                            format!("{}{}", builder, group.chars().skip(1).collect::<String>());
                        break;
                    }
                }
            }
            num = isize::from_str_radix(&builder, 2)
                .unwrap()
                .try_into()
                .unwrap();
        }
        t => {
            let mut nums = vec![];
            match pop(bin_iter, 1).as_str() {
                "0" => {
                    let sub_len: usize = isize::from_str_radix(&pop(bin_iter, 15), 2)
                        .unwrap()
                        .try_into()
                        .unwrap();
                    let iter_len = bin_iter.len();
                    // ret (ver, num) make list of num and solce according to rules
                    while bin_iter.len() != iter_len - sub_len {
                        let (v, n) = parse_package(bin_iter);
                        ver += v;
                        nums.push(n);
                    }
                }
                _ => {
                    let sub_len = isize::from_str_radix(&pop(bin_iter, 11), 2).unwrap();
                    let subs: i64 = sub_len.try_into().unwrap();
                    for _ in 0..subs {
                        let (v, n) = parse_package(bin_iter);
                        ver += v;
                        nums.push(n);
                    }
                }
            }
            match t {
                0 => num = nums.iter().fold(0, |n1, n2| n1 + n2),
                1 => num = nums.iter().fold(1, |n1, n2| n1 * n2),
                2 => {
                    num = *nums
                        .iter()
                        .reduce(|n1, n2| if n1 < n2 { &n1 } else { &n2 })
                        .unwrap()
                }
                3 => {
                    num = *nums
                        .iter()
                        .reduce(|n1, n2| if n1 > n2 { &n1 } else { &n2 })
                        .unwrap()
                }
                5 => num = if nums[0] > nums[1] { 1 } else { 0 },
                6 => num = if nums[0] < nums[1] { 1 } else { 0 },
                7 => num = if nums[0] == nums[1] { 1 } else { 0 },
                _ => panic!(),
            }
        }
    }
    (ver, num)
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let mut ver_sum = 0;
    let hex_to_bin = HashMap::from([
        ('0', String::from("0000")),
        ('1', String::from("0001")),
        ('2', String::from("0010")),
        ('3', String::from("0011")),
        ('4', String::from("0100")),
        ('5', String::from("0101")),
        ('6', String::from("0110")),
        ('7', String::from("0111")),
        ('8', String::from("1000")),
        ('9', String::from("1001")),
        ('A', String::from("1010")),
        ('B', String::from("1011")),
        ('C', String::from("1100")),
        ('D', String::from("1101")),
        ('E', String::from("1110")),
        ('F', String::from("1111")),
    ]);
    let bin_str = get_data()
        .chars()
        .map(|h| hex_to_bin[&h].clone())
        .collect::<String>();
    let mut bin_iter = bin_str.chars().collect::<LinkedList<char>>();
    let mut num = 0;
    while bin_iter.len() >= 8 {
        let (v, n) = parse_package(&mut bin_iter);
        ver_sum += v;
        num += n;
    }

    println!("{}", if part == "part2" { num } else { ver_sum });
}
