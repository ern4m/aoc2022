fn first_part(input: &str) -> String {
    let res = input
        .lines()
        .map(|a| {
            a
                .split(',')
                .map(|[a, b]| [a, b])
        })
        .collect::<Vec<&str, &str>>();
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = first_part(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = second_part(INPUT);
        assert_eq!(result, "70");
    }
}

