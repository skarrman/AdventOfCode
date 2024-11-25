use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Hash, PartialEq, Eq)]
struct P {
    x: i64,
    y: i64,
    z: i64,
}

struct Inst {
    on: bool,
    x_from: i64,
    x_to: i64,
    y_from: i64,
    y_to: i64,
    z_from: i64,
    z_to: i64,
}
fn extract(range: &str) -> Vec<i64> {
    range
        .replace("=", "")
        .replace("x", "")
        .replace("y", "")
        .replace("z", "")
        .split("..")
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn get_instructions() -> Vec<Inst> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    contents
        .split("\n")
        .map(|row| {
            let in_pts = row.split(" ").collect::<Vec<&str>>();
            let pts = in_pts[1]
                .split(",")
                .map(|s| extract(s))
                .collect::<Vec<Vec<i64>>>();
            Inst {
                on: in_pts[0] == "on",
                x_from: pts[0][0],
                x_to: pts[0][1],
                y_from: pts[1][0],
                y_to: pts[1][1],
                z_from: pts[2][0],
                z_to: pts[2][1],
            }
        })
        .collect()
}

// println!(
//     "{} x={}..{},y={}..{},z={}..{}",
//     if inst.on { "on" } else { "off" },
//     inst.x_from,
//     inst.x_to,
//     inst.y_from,
//     inst.y_to,
//     inst.z_from,
//     inst.z_to
// )

fn reduce(val: i64, part1: bool) -> i64 {
    if part1 && (val < -50 || val > 50) {
        0
    } else {
        val
    }
}

fn axis_overlap(f1:i64,t1:i64, f2:i64,t2:i64) -> bool{
    f1 <= f2 && t1 >= && f2 <= 
}
 
fn is_overlapping(i1:Inst, i2:Inst) -> bool{
    let x = i1.x_from < i2.x_from && i1.x_to > i2.x_from
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };
    let part = part == "part1";
    let insts = get_instructions();
    let mut points: HashMap<P, bool> = HashMap::new();
    for inst in insts {
        for x in reduce(inst.x_from, part)..=reduce(inst.x_to, part) {
            for y in reduce(inst.y_from, part)..=reduce(inst.y_to, part) {
                for z in reduce(inst.z_from, part)..=reduce(inst.z_to, part) {
                    points.insert(P { x: x, y: y, z: z }, inst.on);
                }
            }
        }
    }
    let on = points
        .iter()
        .fold(0, |sum, (_, on)| sum + if *on { 1 } else { 0 });
    println!("{}", on);
}
