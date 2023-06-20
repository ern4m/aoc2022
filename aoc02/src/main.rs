use std::env;
use std::fs;
use bimap::{ BiMap, BiHashMap };
use std::collections::HashMap;

// Make function to read input and parse match pairs
fn parse_file(file_path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(file_path).expect("Error while reading file");
    contents.lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let parsed = parse_file(file_path);
    println!("{:?}", parsed);
    // A | X - Rock
    // B | Y - Paper
    // C | Z - Scissors
    let mut equivalences: BiHashMap<&str, &str> = BiMap::new();
    equivalences.insert("A", "X");
    equivalences.insert("B", "Y");
    equivalences.insert("C", "Z");

    // rules hash designs who wins who
    let mut rules = HashMap::new();
    rules.insert("A", "Z");
    rules.insert("B", "X");
    rules.insert("C", "Y");
}
