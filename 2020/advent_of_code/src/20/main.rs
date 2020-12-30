use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const TILE_SIDE: usize = 10;

#[derive(Debug, Clone, PartialEq)]
enum Flip {
    No,
    Hor,
    Ver,
}

#[derive(Debug, Clone)]
struct Tile {
    id: i64,
    data: Vec<Vec<char>>,
    flip: Flip,
    rot: usize,
}

impl Tile {
    fn flip(&mut self, flip: Flip) {
        self.flip = flip;
    }
    fn rotate(&mut self) {
        self.rot += 1;
        self.rot %= 4;
    }

    fn get_i_j(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        let tile_side = self.data.len();
        for _ in 0..self.rot {
            let (_i, _j) = (i, j);
            i = tile_side - _j - 1;
            j = _i;
        }
        match self.flip {
            Flip::Hor => {
                j = if j < tile_side / 2 {
                    tile_side / 2 + (tile_side / 2 - j) - 1
                } else {
                    tile_side / 2 - (j - tile_side / 2) - 1
                };
            }
            Flip::Ver => {
                i = if i < tile_side / 2 {
                    tile_side / 2 + (tile_side / 2 - i) - 1
                } else {
                    tile_side / 2 - (i - tile_side / 2) - 1
                };
            }
            _ => (),
        };
        (i, j)
    }

    fn get(&self, i: usize, j: usize) -> char {
        let (i, j) = self.get_i_j(i, j);
        self.data[i][j]
    }

    fn set(&mut self, i: usize, j: usize, c: char) {
        let (i, j) = self.get_i_j(i, j);
        self.data[i][j] = c;
    }

    fn print(&self) {
        for i in 0..self.data.len() {
            let mut row = "".to_string();
            for j in 0..self.data[i].len() {
                row = format!("{}{}", row, self.get(i, j));
            }
            println!("{}", row);
        }
        println!("");
    }
}

impl Eq for Tile {}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..TILE_SIDE {
            for j in 0..TILE_SIDE {
                if self.get(i, j) != other.get(i, j) {
                    return false;
                }
            }
        }
        true
    }
}

fn get_tiles() -> Vec<Tile> {
    let path = "src/20/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .split("\n\n")
        .map(|tile_block| {
            let mut id_split = tile_block.split("\n");
            let id = id_split
                .next()
                .unwrap()
                .replace("Tile ", "")
                .replace(":", "")
                .parse()
                .unwrap();
            let data = id_split.map(|row| row.chars().collect()).collect();
            Tile {
                id: id,
                data: data,
                flip: Flip::No,
                rot: 0,
            }
        })
        .collect()
}
// #[derive(Clone, Copy, Debug, Hash)]
// enum Side {
//     Right,
//     Down,
//     Left,
//     Up,
// }

// impl Eq for Side {}
// impl PartialEq for Side {
//     fn eq(&self, other: &Self) -> bool {
//         *self == *other
//     }
// }

fn find_match(fixed: &Tile, victim: &mut Tile) -> bool {
    for _ in 0..4 {
        // Rotations
        victim.rotate();
        for _ in 0..3 {
            // Flippes
            match victim.flip {
                Flip::No => victim.flip(Flip::Hor),
                Flip::Hor => victim.flip(Flip::Ver),
                Flip::Ver => victim.flip(Flip::No),
            }
            let (mut right_match, mut bottom_match, mut left_match, mut top_match) =
                (true, true, true, true);
            for i in 0..TILE_SIDE {
                if fixed.get(i, TILE_SIDE - 1) != victim.get(i, 0) {
                    right_match = false;
                }
                if fixed.get(TILE_SIDE - 1, i) != victim.get(0, i) {
                    bottom_match = false;
                }
                if fixed.get(0, i) != victim.get(TILE_SIDE - 1, i) {
                    left_match = false;
                }
                if fixed.get(0, i) != victim.get(TILE_SIDE - 1, i) {
                    top_match = false;
                }
                if !(right_match || bottom_match || left_match || top_match) {
                    break;
                }
            }
            if right_match || bottom_match || left_match || top_match {
                return true;
            }
        }
    }
    false
}

fn find_sea_monster(image_tile: &mut Tile, sea_monster: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut sea_monster_start_index = Vec::new();
    for _ in 0..4 {
        //Rot
        image_tile.rotate();
        for _ in 0..3 {
            // Flippes
            match image_tile.flip {
                Flip::No => image_tile.flip(Flip::Hor),
                Flip::Hor => image_tile.flip(Flip::Ver),
                Flip::Ver => image_tile.flip(Flip::No),
            }
            for i in 0..image_tile.data.len() - sea_monster.len() {
                for j in 0..image_tile.data.len() - sea_monster[0].len() {
                    let mut is_sea_monster = true;
                    for s_i in 0..sea_monster.len() {
                        for s_j in 0..sea_monster[s_i].len() {
                            if sea_monster[s_i][s_j] == '#'
                                && image_tile.get(i + s_i, j + s_j) != '#'
                            {
                                is_sea_monster = false;
                            }
                        }
                    }
                    if is_sea_monster {
                        sea_monster_start_index.push((i, j));
                    }
                }
            }
            if sea_monster_start_index.len() != 0 {
                return sea_monster_start_index;
            }
        }
    }
    panic!();
}

fn fix_orientation(tiles: &mut Vec<Tile>, below: usize, middle: usize, right: usize) {
    for _ in 0..4 {
        // Rotationss
        tiles[middle].rotate();
        for _ in 0..3 {
            // Flippes
            match tiles[middle].flip {
                Flip::No => tiles[middle].flip(Flip::Hor),
                Flip::Hor => tiles[middle].flip(Flip::Ver),
                Flip::Ver => tiles[middle].flip(Flip::No),
            }
            if check(tiles, middle, below, true) && check(tiles, middle, right, false) {
                return;
            }
        }
    }
    panic!("Orientation not fixed!");
}

fn check(tiles: &mut Vec<Tile>, fixed: usize, tile: usize, is_below: bool) -> bool {
    for _ in 0..4 {
        // Rotations
        tiles[tile].rotate();
        for _ in 0..3 {
            // Flippes
            match tiles[tile].flip {
                Flip::No => tiles[tile].flip(Flip::Hor),
                Flip::Hor => tiles[tile].flip(Flip::Ver),
                Flip::Ver => tiles[tile].flip(Flip::No),
            }
            if (is_below
                && (0..TILE_SIDE).fold(true, |res, j| {
                    res && tiles[fixed].get(TILE_SIDE - 1, j) == tiles[tile].get(0, j)
                }))
                || (!is_below
                    && (0..TILE_SIDE).fold(true, |res, i| {
                        res && tiles[fixed].get(i, TILE_SIDE - 1) == tiles[tile].get(i, 0)
                    }))
            {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut tiles = get_tiles();
    let side_length = (tiles.len() as f64).sqrt() as usize;
    let mut matches: HashMap<i64, HashSet<i64>> = HashMap::new();
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let mut match_found = false;
            for _ in 0..4 {
                // Rotations
                if match_found {
                    break;
                }
                tiles[i].rotate();
                for _ in 0..3 {
                    // Flippes
                    match tiles[i].flip {
                        Flip::No => tiles[i].flip(Flip::Hor),
                        Flip::Hor => tiles[i].flip(Flip::Ver),
                        Flip::Ver => tiles[i].flip(Flip::No),
                    }
                    if find_match(&tiles.clone()[i], &mut tiles[j]) {
                        matches
                            .entry(tiles[i].id)
                            .or_insert(HashSet::new())
                            .insert(tiles[j].id);
                        matches
                            .entry(tiles[j].id)
                            .or_insert(HashSet::new())
                            .insert(tiles[i].id);
                        match_found = true;
                        break;
                    }
                }
            }
        }
    }
    let mut corner_ids = matches
        .iter()
        .filter(|(_, ms)| ms.len() == 2)
        .map(|(&id, _)| id)
        .collect::<Vec<i64>>();

    println!(
        "First problem: {}",
        &corner_ids.iter().fold(1, |p, id| p * id)
    );
    let mut image_of_ids = vec![vec![1111i64; side_length]; side_length];
    let mut assigned_ids: HashSet<i64> = HashSet::new();
    // Set one corner
    let mut curr_id = corner_ids.pop().unwrap();
    image_of_ids[0][0] = curr_id;
    assigned_ids.insert(curr_id);
    for j in 1..side_length {
        curr_id = *matches[&curr_id]
            .iter()
            .filter(|&id| matches[id].len() < 4 && !assigned_ids.contains(id))
            .next()
            .unwrap();
        image_of_ids[0][j] = curr_id;
        assigned_ids.insert(curr_id);
    }
    for i in 1..side_length {
        curr_id = *matches[&image_of_ids[i - 1][0]]
            .iter()
            .filter(|&id| !assigned_ids.contains(id))
            .next()
            .unwrap();
        image_of_ids[i][0] = curr_id;
        assigned_ids.insert(curr_id);
        for j in 1..side_length {
            curr_id = *matches[&curr_id]
                .intersection(&matches[&image_of_ids[i - 1][j]])
                .filter(|&id| !assigned_ids.contains(id))
                .next()
                .unwrap();
            image_of_ids[i][j] = curr_id;
            assigned_ids.insert(curr_id);
        }
    }
    let tiles_map = &tiles
        .iter()
        .zip(0..tiles.len())
        .fold(HashMap::new(), |mut map, (tile, i)| {
            map.insert(tile.id, i);
            map
        });
    for i in 0..image_of_ids.len() - 1 {
        for j in 0..image_of_ids[i].len() - 1 {
            let (right, middle, below) = (
                image_of_ids[i][j + 1],
                image_of_ids[i][j],
                image_of_ids[i + 1][j],
            );
            fix_orientation(
                &mut tiles,
                tiles_map[&below],
                tiles_map[&middle],
                tiles_map[&right],
            );
        }
        check(
            &mut tiles,
            tiles_map[&image_of_ids[i][image_of_ids[i].len() - 2]],
            tiles_map[&image_of_ids[i][image_of_ids[i].len() - 1]],
            false,
        );
    }
    for j in 0..image_of_ids.len() {
        check(
            &mut tiles,
            tiles_map[&image_of_ids[image_of_ids.len() - 2][j]],
            tiles_map[&image_of_ids[image_of_ids.len() - 1][j]],
            true,
        );
    }

    let mut border_less_data: Vec<Vec<char>> = Vec::new();
    for i in 0..image_of_ids.len() * TILE_SIDE {
        let mut row_data: Vec<char> = Vec::new();
        for j in 0..image_of_ids.len() * TILE_SIDE {
            let (im_i, im_j) = (i / TILE_SIDE, j / TILE_SIDE);
            let (px_i, px_j) = (i % TILE_SIDE, j % TILE_SIDE);
            let c = tiles[tiles_map[&image_of_ids[im_i][im_j]]].get(px_i, px_j);
            if px_i != 0 && px_i != TILE_SIDE - 1 && px_j != 0 && px_j != TILE_SIDE - 1 {
                row_data.push(c);
            }
        }
        if row_data.len() != 0 {
            border_less_data.push(row_data);
        }
    }

    let mut image_tile = Tile {
        id: 0,
        data: border_less_data,
        rot: 0,
        flip: Flip::No,
    };

    let sea_monster: Vec<Vec<char>> = vec![
        vec![
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', '#', ' ',
        ],
        vec![
            '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ',
            '#', '#', '#',
        ],
        vec![
            ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#',
            ' ', ' ', ' ',
        ],
    ];

    let sea_monsters = find_sea_monster(&mut image_tile, &sea_monster);
    for (i, j) in sea_monsters {
        for s_i in 0..sea_monster.len() {
            for s_j in 0..sea_monster[s_i].len() {
                if sea_monster[s_i][s_j] == '#' {
                    image_tile.set(i + s_i, j + s_j, 'O');
                }
            }
        }
    }

    let mut num = 0;
    for i in 0..image_tile.data.len() {
        for j in 0..image_tile.data[i].len() {
            num += if image_tile.get(i, j) == '#' { 1 } else { 0 };
        }
    }
    println!("Second problem: {}", num);
}
