use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn get_data() -> Vec<(HashSet<String>, Vec<String>)> {
    let path = "src/21/input.txt";
    let file_contents = fs::read_to_string(path).expect("Could not read file");
    file_contents
        .replace(")", "")
        .split("\n")
        .map(|row| {
            let allergens_split = row.split(" (contains ").collect::<Vec<&str>>();
            let ingredients = allergens_split[0]
                .split(" ")
                .map(|ingredient| ingredient.to_string())
                .collect();
            (
                ingredients,
                if allergens_split.len() > 1 {
                    allergens_split[1]
                        .split(", ")
                        .map(|allergen| allergen.to_string())
                        .collect()
                } else {
                    Vec::new()
                },
            )
        })
        .collect()
}

fn map_al_to_in(
    data: &Vec<(HashSet<String>, Vec<String>)>,
) -> HashMap<String, Vec<HashSet<String>>> {
    let mut al_to_in: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    for (ins, als) in data {
        for al in als {
            al_to_in
                .entry(al.to_string())
                .or_insert(Vec::new())
                .push(ins.clone());
        }
    }
    al_to_in
}

fn all_ings(data: &Vec<(HashSet<String>, Vec<String>)>) -> HashSet<String> {
    let mut all_ings = HashSet::new();
    for (ings, _) in data {
        all_ings.extend(ings.clone());
    }
    all_ings
}

fn potential_allergens(
    al_to_in: &HashMap<String, Vec<HashSet<String>>>,
) -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let mut pot_al = HashSet::new();
    let mut pot_al_ing: HashMap<String, HashSet<String>> = HashMap::new();
    for (al, inss) in al_to_in {
        let inter = inss[0]
            .iter()
            .filter(|ing| inss.iter().fold(true, |res, ins| res && ins.contains(*ing)))
            .map(|ing| ing.to_string())
            .collect::<HashSet<String>>();
        pot_al_ing.insert(al.to_string(), inter.clone());
        pot_al.extend(inter);
    }
    (pot_al, pot_al_ing)
}

fn main() {
    let data = get_data();
    let all = all_ings(&data);
    let (pot_al, mut pot_al_ing) = potential_allergens(&map_al_to_in(&data));
    let diff = all.difference(&pot_al).collect::<HashSet<_>>();

    let occ = diff.iter().fold(0, |sum, &ing| {
        sum + data
            .iter()
            .fold(0, |s, (ings, _)| s + if ings.contains(ing) { 1 } else { 0 })
    });
    println!("First problem: {}", occ);
    let mut dang_ingr: Vec<(String, String)> = Vec::new();
    loop {
        match pot_al_ing
            .clone()
            .iter()
            .filter(|(_, set)| set.len() == 1)
            .next()
        {
            Some((al, singleton)) => {
                let ing = singleton.iter().next().unwrap();
                pot_al_ing = pot_al_ing
                    .iter()
                    .filter(|set| set.0 != al)
                    .map(|(key, set)| {
                        (
                            key.clone(),
                            set.iter()
                                .filter(|&val| val != ing)
                                .map(|val| val.to_string())
                                .collect::<HashSet<String>>(),
                        )
                    })
                    .collect::<HashMap<String, HashSet<String>>>();
                dang_ingr.push((al.clone(), ing.clone()));
            }
            _ => break,
        }
    }
    dang_ingr.sort_by(|(al1, _), (al2, _)| al1.cmp(al2));
    let mut canonical_dangerous_ingredient_list = "".to_string();
    for (_, ing) in dang_ingr {
        canonical_dangerous_ingredient_list =
            format! {"{},{}", canonical_dangerous_ingredient_list, ing};
    }
    canonical_dangerous_ingredient_list = canonical_dangerous_ingredient_list
        .chars()
        .skip(1)
        .take(canonical_dangerous_ingredient_list.len())
        .collect::<String>();
    println!("Second problem: {}", canonical_dangerous_ingredient_list);
}
