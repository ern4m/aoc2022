use std::env;
use std::any::Any;
use std::str::Split;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Unable to read file!");

    let biggest = 0;



    let sections = contents.split("\n");
    for section in sections {
        let current: u32 = 0;
        let mut newInventory: bool = false;
        match section {
            "" |" "| "\n" => { &newInventory = true }
        }
        let parsed_section: u32 = section.parse().unwrap();
        println!("Inventories = {}", section)
    }
}

