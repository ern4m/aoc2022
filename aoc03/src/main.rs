use std::collections::HashMap;

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

fn main() {
    // let mymap = make_map();
    let mymap = make_map();
    
    let phrase = "asdmnzcklasehjweIKSDBAJBabDBsaldhbASH";

    let paddr = phrase.as_ptr();
    dbg!(paddr);
    println!("{:p}", paddr);

    unsafe {
        println!("{:p}", paddr.offset(4));
        println!("{:p}", &paddr.offset(4));
    }

    // dbg!(phrase.split_at(phrase.len()/2));
}
