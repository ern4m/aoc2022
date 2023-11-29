use std::collections::LinkedList;
//Vec<LinkedList<&str>>
fn parse_input(input: &str) -> Vec<&str> {
   //let input = input.lines().map(|line| line).collect::<Vec<&str>>();
   let input = input.split("\n\n").collect::<Vec<&str>>();
   input
}

fn parse_stack(input: Vec<&str>) -> () {
    let parsed: Vec<&str> = input[0].lines().rev().collect();
    //dbg!(&parsed);
    let mut res: Vec<Option<&str>> = Vec::new();
    let mut _count = 0;
    let _ = parsed
        .into_iter().map(|line| {
            let _ = line.split(" ")
                .map(|col| {
                    let col = col.trim();
                    dbg!(col);
                    if col == "" {
                        _count += 1;
                        if _count == 4 {
                            _count = 0;
                            res.push(None) 
                        } else {
                            ()
                        }
                    } else {
                        _count = 0;
                        res.push(Some(col))
                    }
                });
        });
    dbg!(res);
    ()
}

pub fn first_part(input: &str) {
    let input = parse_input(input);
    parse_stack(input);
    ()
}

pub fn second_part(input: &str) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part1_works() {
        let result = first_part(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = second_part(INPUT);
        assert_eq!(result, 4);
    }
}

