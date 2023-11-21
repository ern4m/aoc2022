use std::fs;
use aoc04::first_part;

fn main() {
    let file = fs::read_to_string("./new_input.txt");
    match file {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    };
    //println!("{}", first_part(&file));
}

