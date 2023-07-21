use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = match s.to_lowercase().as_str() {
            "a" | "x" => Move::Rock,
            "b" | "y" => Move::Paper,
            "c" | "z" => Move::Scissors,
            _ => return Err(ParseMoveError),
        };
        Ok(m)
    }
}

impl Move {
    pub fn vs(&self, opponent: &Move) -> Outcome {
        if self.defeats().eq(opponent) {
            Outcome::Win
        } else if self.defeated().eq(opponent) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    pub fn defeats(&self) -> Move {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    pub fn defeated(&self) -> Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}
