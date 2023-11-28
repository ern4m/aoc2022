use std::fs;
use aoc04::first_part;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", first_part(&file));
}

