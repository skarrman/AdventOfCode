use std::fs;

fn get_data() -> Vec<Vec<char>> {
    let path = "src/03/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n")
        .map(|row| row.chars().collect())
        .collect()
}

fn search_slope(map: &Vec<Vec<char>>, (dx, dy): (usize, usize)) -> usize {
    let (mut trees, mut x, mut y) = (0, 0, 0);
    while y < map.len() {
        if map[y][x] == '#' {
            trees += 1;
        }
        x = (x + dx) % map[y].len();
        y += dy;
    }
    trees
}

fn main() {
    let map = get_data();
    let fst_trees = search_slope(&map, (3, 1));
    println!("First challenge: {}", fst_trees);
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let snd_trees = slopes.iter().fold(1, |prd, &d| prd * search_slope(&map, d));
    println!("Second challenge: {}", snd_trees);
}
