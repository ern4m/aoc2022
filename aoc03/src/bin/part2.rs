use std::fs;

use aoc03::second_part;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    dbg!(second_part(&file));
}

