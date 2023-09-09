use std::fs;

use aoc03::first_part;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", first_part(&file));
}
