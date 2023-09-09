#![feature(iter_array_chunks)]
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

pub fn second_part(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .array_chunks()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| {
                    b.contains(*a_char)
                        && c.contains(*a_char)
                })
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = first_part(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = second_part(INPUT);
        assert_eq!(result, "70");
    }
}
