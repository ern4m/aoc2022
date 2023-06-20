use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Unable to read file!");

    let mut most_calories = 0;

    let sections = contents.split("\n");
    let mut inventory_sum: i32 = 0;
    for section in sections {
        
        match section {
            "" |" " => {inventory_sum = 0},
            _ => {
                inventory_sum += section.parse::<i32>().unwrap()
            },
        }
        println!("Inventory sum: {}", inventory_sum);
        if inventory_sum > most_calories {
            most_calories = inventory_sum
        }
    }
    println!("{most_calories}");
}

