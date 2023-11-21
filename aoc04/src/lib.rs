fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|v| {
            v.split(&[',', '-'])
                .map(|v| {
                    v.parse().unwrap()}
                    )
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn check_if_contains(vec: Vec<i32>) -> bool {
    let _pairs = vec.clone();
    match _pairs[..] {
        [a, b, c, d] => {
            if (a >= c && a <= d) && (b >= c && b <= d) {
                //println!("testing first pair inside second");
                return true
            } else if c >= a {
                let _reverse_pairs: Vec<i32> = vec![c, d, a, b];
                //println!("testing second pair inside first");
                return check_if_contains(_reverse_pairs)
            } else {
                return false
            }
        },
        _ => return false,
    }
}


pub fn first_part(input: &str) -> i32 {
    let mut counter = 0;

    for x in parse_input(input) {
        if check_if_contains(x) {
            counter += 1;
        }
    }
    counter
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
        assert_eq!(result, 2);
    }

}

