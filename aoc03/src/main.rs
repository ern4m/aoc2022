use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn make_map() -> HashMap<char, i32> {
    let lower = 'a'..'z';
    let lower_prior = 1..26;
    let upper = 'A'..'Z';
    let upper_prior = 27..52;

    let mut lmap: HashMap<char, i32> = lower
        .zip(lower_prior)
        .map(|(char_val, num_val)| (char_val, num_val))
        .collect();

    let umap: HashMap<char, i32> = upper
        .zip(upper_prior)
        .map(|(char_val, num_val)| (char_val, num_val))
        .collect();

    lmap.extend(umap);

    lmap
}


fn get_line_result(line: &str, map: &HashMap<char, i32>) -> i32{

    let (start, end) = line.split_at(line.len()/2);
    
    let mut result = 0;

    for char in start.chars() {
       if end.contains(char) {
           result = map.get(&char).unwrap();
       } else {
           result = 0;
       }
    }

    result

}

fn main() -> std::io::Result<()> {
    // let mymap = make_map();
    let mymap = make_map();

    for line in include_str!("../input.txt").lines() {
        get_line_result(&line, &mymap);
    }

    Ok(())
}
