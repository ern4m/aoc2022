use std::env;
use std::fs;
use bimap::{ BiMap, BiHashMap };
use std::collections::HashMap;

// Make function to read input and parse match pairs
fn parse_file(file_path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(file_path).expect("Error while reading file");
    contents
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let parsed: Vec<Vec<String>> = parse_file(file_path);
    println!("{:?}", parsed.len());

    // A | X - Rock
    // B | Y - Paper
    // C | Z - Scissors
    let mut equivalences: BiHashMap<&str, &str> = BiMap::new();
    equivalences.insert("A", "X");
    equivalences.insert("B", "Y");
    equivalences.insert("C", "Z");

    // rules hash designs who wins who
    // Win = 6 | Draw = 3 | Loss = 0
    // Scissors = 3 | Paper = 2 | Rock = 1
    let mut rules = HashMap::new();
    rules.insert("X", 1);
    rules.insert("Y", 2);
    rules.insert("Z", 3);

    // map of who left side wins
    let mut comparisson_rules = HashMap::new();
    comparisson_rules.insert("A", "C");
    comparisson_rules.insert("B", "A");
    comparisson_rules.insert("C", "B");

    let mut counter = 0;

    for choice in parsed {
        let opp = choice[0].as_str();
        let our = equivalences.get_by_right(choice[1].as_str()).unwrap();

        // inserting value of match
        if &opp == our {
            counter += 3;
        } else if comparisson_rules.get(our).unwrap() == &opp {
            counter += 6;
        }

        // inserting value of choice
        counter += rules.get(choice[1].as_str()).unwrap();
    }
    println!("{}", counter);
}
