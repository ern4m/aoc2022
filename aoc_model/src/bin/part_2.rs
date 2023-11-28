use std::fs;
use aoc04::second_part;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", second_part(&file));
}

