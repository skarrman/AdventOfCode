use std::fs;

#[derive(Clone, PartialEq)]
enum Dir {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}
#[derive(Clone, PartialEq)]
enum Act {
    D(Dir),
    Forw,
    Rot,
}

struct Inst {
    a: Act,
    n: i32,
}

fn get_data() -> Vec<Inst> {
    let path = "src/12/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| {
            let letter = row.chars().nth(0).unwrap();
            let num = row
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            match letter {
                n if ['E', 'S', 'W', 'N'].contains(&n) => Inst {
                    a: Act::D(match n {
                        'E' => Dir::EAST,
                        'S' => Dir::SOUTH,
                        'W' => Dir::WEST,
                        'N' => Dir::NORTH,
                        _ => panic!("Unknown dir"),
                    }),
                    n: num,
                },
                'F' => Inst {
                    a: Act::Forw,
                    n: num,
                },
                'R' => Inst {
                    a: Act::Rot,
                    n: if num == 270 { 3 } else { 360 / (360 - num) },
                },
                'L' => Inst {
                    a: Act::Rot,
                    n: if num == 90 { 3 } else { 360 / num },
                },
                _ => panic!("Unknown letter"),
            }
        })
        .collect()
}

fn drive(dir: Dir, n: i32) -> (i32, i32) {
    let (x, y) = match dir {
        Dir::EAST => (1, 0),
        Dir::SOUTH => (0, 1),
        Dir::WEST => (-1, 0),
        Dir::NORTH => (0, -1),
    };
    (x * n, y * n)
}

fn main() {
    let dirs = vec![Dir::EAST, Dir::SOUTH, Dir::WEST, Dir::NORTH];
    let mut dir = 0;
    let (mut x, mut y) = (0, 0);
    for inst in get_data() {
        let (_x, _y) = match inst.a {
            Act::D(d) => drive(d, inst.n),
            Act::Rot => {
                dir = (dir + inst.n) % (dirs.len() as i32);
                (0, 0)
            }
            Act::Forw => drive(dirs[dir as usize].clone(), inst.n),
        };
        x += _x;
        y += _y;
    }
    println!("First problem: {}", x.abs() + y.abs());

    let (mut wx, mut wy) = (10, -1);
    x = 0;
    y = 0;
    for inst in get_data() {
        if inst.a == Act::Forw {
            x += wx * inst.n;
            y += wy * inst.n;
        } else {
            let (_wx, _wy) = match inst.a {
                Act::D(d) => {
                    let (dwx, dwy) = drive(d, inst.n);
                    (wx + dwx, wy + dwy)
                }
                Act::Rot => match inst.n {
                    1 => (-wy, wx),
                    2 => (-wx, -wy),
                    3 => (wy, -wx),
                    _ => panic!("Unknown degree!"),
                },
                _ => panic!("Unknown instruction!"),
            };
            wx = _wx;
            wy = _wy;
        }
    }
    println!("Second problem: {}", x.abs() + y.abs());
}
