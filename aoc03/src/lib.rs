use std::collections::HashMap;

pub fn first_part(input: &str) -> String {

    let mapping: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect();

    let result: usize = input
        .lines()
        .map(|line| {
            let len = line.len() / 2;
            let first = &line[0..len];
            let second = &line[len..(len*2)];

            // let (first, second) = line.split_at(line.len() / 2);

            let common_char = first
                .chars()
                .find(|c| second.contains(*c))
                .unwrap();
            mapping.get(&common_char).unwrap()
        })
        .sum::<usize>();
    
    result.to_string()
}
