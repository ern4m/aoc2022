fn parse_input(input: &str) -> Vec<[i32; 4]> {
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

// fn check_if_overlaps(vec: Vec<i32>) -> bool {
//     let _pairs = vec.clone();
//     match _pairs[..] {
//         [a, b, c, d] => {
//             // let fp = a..=b;
//             if a <= c && b <= d {
//                 return true
//             } else if c >= a {
//                 let _reverse_pairs: Vec<i32> = vec![c, d, a, b];
//                 return check_if_overlaps(_reverse_pairs)
//             } else {
//                 return false
//             }
//         },
//         _ => return false,
//     }
// }

fn check_if_overlaps(vec: Vec<i32>) -> bool {
    let _pairs = vec.clone();
    match _pairs[..] {
        [a, b, c, d] => {
            let fp = a..=b;
            // let sp = c..=d;
            if fp.contains(&c) || fp.contains(&d) {
                return true
            } else if c >= a {
                let _reverse_pairs: Vec<i32> = vec![c, d, a, b];
                return check_if_overlaps(_reverse_pairs)
            } else {
                return false
            }
        },
        _ => false
    }
}


pub fn first_part(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|[a, b, c, d]| (a >= c && b <= d) || (c >= a && d <= b))
        .count()
}

pub fn second_part(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|[a, b, c, d]| (a <= c && b >= c) || (c <= a && d >= a))
        .count()
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

    #[test]
    fn part2_works() {
        let result = second_part(INPUT);
        assert_eq!(result, 4);
    }
}

