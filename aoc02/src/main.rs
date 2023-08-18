
// implementing the data structures necessary for our data manipulation necessities

use std::str::FromStr;
use itertools::{process_results, Itertools};

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, theirs:Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }
        
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    // setting all moves
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    // returns move value
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    
    // returns result of matchs given 2 moves
    // fn outcome(self, theirs: Move) -> Outcome {
    //     match(self, theirs) {
    //         (Move::Rock, Move::Paper) => Outcome::Loss,
    //         (Move::Rock, Move::Rock) => Outcome::Draw,
    //         (Move::Rock, Move::Scissors) => Outcome::Win,
    //         (Move::Paper, Move::Scissors) => Outcome::Loss,
    //         (Move::Paper, Move::Paper) => Outcome::Draw,
    //         (Move::Paper, Move::Rock) => Outcome::Win,
    //         (Move::Scissors, Move::Rock) => Outcome::Loss,
    //         (Move::Scissors, Move::Scissors) => Outcome::Draw,
    //         (Move::Scissors, Move::Paper) => Outcome::Win,
    //     }
    //     
    // }

    // return what move defeats the other
    // Self::Rock.beats(Self::Paper) -> returns false beacuse there's no match for this case
    fn beats(self, other: Move) -> bool {
        matches!((self, other), (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) |  (Self::Scissors, Self::Paper))
    }

    // return outcome given the comparisson of entries based on beats return
    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    // check winning move
    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("At least one move should beat us! Something went wrong!")
    }
    
    // check losing move
    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("We should beat at least one move! Something went wrong!")
    }

    fn drawing_move(self) -> Self {
        self
    }
}

// implementing TryFrom for Move, so we can parse from the chars in input file and make them chars
// into ours types
impl TryFrom<char> for Move {
    type Error = color_eyre::Report;
    //  first phase parsing accepting ABCXYZ as input
    //    fn try_from(value: char) -> Result<Self, Self::Error> {
    //        match value {
    //            'A' | 'X' => Ok(Move::Rock),
    //            'B' | 'Y' => Ok(Move::Paper),
    //            'C' | 'Z' => Ok(Move::Scissors),
    //            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
    //        }
    //    }

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    // returns outcome based on ours and theirs move
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

// implementing FromStr to our Round, so we can parse a round given an &str (a line):
impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // getting chars from the string(line) input
        let mut chars = s.chars();

        // let () = () else {}
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);


        // here rust knows that theirs should be a Move, because the struct indicates it, so we need
        // to turn theirs(char) into our Move, to do so we need to call the TryFrom implementation
        // that we created before, returning this way our Moves for each player
        Ok (Self { theirs, ours })
    }

}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
// // an way to do it iterating with loop
//    let mut total_score = 0;
//    for round in include_str!("../input.txt").lines().map(Round::from_str) {
//            total_score += round?.our_score();
//    }

// // another way of doing it using functional elements from itertools
// // like process_results that receives an iterable Item that's a Result and a 
// // processor of type FnOnce
//    let total_score: usize = process_results(
//        include_str!("../input.txt")
//        .lines()
//        .map(Round::from_str)
//        .map_ok(|r| r.our_score()),
//        |it| it.sum(),
//    )?;

    let total_score:usize = process_results(
        include_str!("../input.txt")
            .lines()
            .map(Round::from_str)
            .map_ok(|r| dbg!(dbg!(r).our_score())),
        |it| it.sum()
    )?;

    dbg!(total_score);

    Ok(())
}
