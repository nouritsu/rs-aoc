use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
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
        match self {
            Move::Rock => {
                if opponent.eq(&Move::Scissors) {
                    Outcome::Win
                } else if opponent.eq(&Move::Paper) {
                    Outcome::Loss
                } else {
                    Outcome::Draw
                }
            }
            Move::Paper => {
                if opponent.eq(&Move::Rock) {
                    Outcome::Win
                } else if opponent.eq(&Move::Scissors) {
                    Outcome::Loss
                } else {
                    Outcome::Draw
                }
            }
            Move::Scissors => {
                if opponent.eq(&Move::Paper) {
                    Outcome::Win
                } else if opponent.eq(&Move::Rock) {
                    Outcome::Loss
                } else {
                    Outcome::Draw
                }
            }
        }
    }
}
