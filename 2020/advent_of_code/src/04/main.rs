use std::collections::HashMap;
use std::fs;
extern crate regex;

fn get_data() -> Vec<HashMap<String, String>> {
    let path = "src/04/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");

    let re = regex::Regex::new(r"\n|\s").unwrap();
    file_contents
        .split("\n\n")
        .map(|row| {
            let mut map = HashMap::new();
            let _: Vec<Option<String>> = re
                .split(row)
                .map(|parts| {
                    let mut val = parts.split(":");
                    let (key, val) = (val.next().unwrap(), val.next().unwrap());
                    map.insert(String::from(key), String::from(val))
                })
                .collect();
            map
        })
        .collect()
}

fn parse_i32(i: Option<&String>) -> i32 {
    i.unwrap().parse::<i32>().unwrap()
}

fn valid_i32(val: i32, min: i32, high: i32) -> bool {
    min <= val && val <= high
}

fn valid_hgt(hgt: &String) -> bool {
    if hgt.contains("cm") || hgt.contains("in") {
        let (min, high) = if hgt.contains("cm") {
            (150, 193)
        } else {
            (59, 76)
        };
        let height = hgt
            .replace("cm", "")
            .replace("in", "")
            .parse::<i32>()
            .unwrap();
        valid_i32(height, min, high)
    } else {
        false
    }
}

fn valid_hcl(hcl: &String) -> bool {
    let re = regex::Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    re.is_match(hcl)
}

fn valid_ecl(ecl: &String) -> bool {
    let ecls = vec![
        String::from("amb"),
        String::from("blu"),
        String::from("brn"),
        String::from("gry"),
        String::from("grn"),
        String::from("hzl"),
        String::from("oth"),
    ];
    ecls.contains(ecl)
}

fn valid_pid(pid: &String) -> bool {
    pid.len() == 9
}

fn main() {
    let mut data = get_data();
    let fields: Vec<String> = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];

    data.retain(|row| {
        fields
            .iter()
            .fold(true, |val, key| val && row.contains_key(key))
    });
    println!("First challenge: {}", data.len());

    let valid_snd = data
        .iter()
        .map(|row| {
            let byr = parse_i32(row.get(&fields[0]));
            let iyr = parse_i32(row.get(&fields[1]));
            let eyr = parse_i32(row.get(&fields[2]));
            let hgt = row.get(&fields[3]).unwrap();
            let hcl = row.get(&fields[4]).unwrap();
            let ecl = row.get(&fields[5]).unwrap();
            let pid = row.get(&fields[6]).unwrap();
            valid_i32(byr, 1920, 2002)
                && valid_i32(iyr, 2010, 2020)
                && valid_i32(eyr, 2020, 2030)
                && valid_hgt(hgt)
                && valid_hcl(hcl)
                && valid_ecl(ecl)
                && valid_pid(pid)
        })
        .fold(0, |valids, valid| valids + if valid { 1 } else { 0 });
    println!("Second challenge: {}", valid_snd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byr() {
        assert!(valid_i32(2002, 1920, 2002));
        assert!(!valid_i32(2003, 1920, 2002));
    }
    #[test]
    fn hgt_val() {
        assert!(valid_hgt(&String::from("60in")));
        assert!(valid_hgt(&String::from("190cm")));
    }
    #[test]
    fn hgt_inval() {
        assert!(!valid_hgt(&String::from("190in")));
        assert!(!valid_hgt(&String::from("190")));
    }
    #[test]
    fn hcl() {
        assert!(valid_hcl(&String::from("#123abc")));
        assert!(!valid_hcl(&String::from("#123abz")));
        assert!(!valid_hcl(&String::from("123abc")));
    }
    #[test]
    fn ecl() {
        assert!(valid_ecl(&String::from("brn")));
        assert!(!valid_ecl(&String::from("wat")));
    }
    #[test]
    fn pid() {
        assert!(valid_pid(&String::from("000000001")));
        assert!(!valid_pid(&String::from("0123456789")));
    }
}
