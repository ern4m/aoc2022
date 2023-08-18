
// implementing the data structures necessary for our data manipulation necessities

use std::str::FromStr;

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
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    
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
}

// implementing TryFrom for Move, so we can parse from the chars in input file and make them chars
// into ours types
impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
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
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        // here rust knows that theirs should be a Move, because the struct indicates it, so we need
        // to turn theirs(char) into our Move, to do so we need to call the TryFrom implementation
        // that we created before, returning this way our Moves for each player
        Ok (Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }

}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rounds: Vec<Round> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_,_>>()?;
    
    let total_score: usize = rounds.iter().map(|r| r.our_score()).sum();

    dbg!(total_score);

    Ok(())
}
