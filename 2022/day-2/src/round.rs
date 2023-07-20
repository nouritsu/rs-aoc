use std::str::FromStr;

use crate::r#move::{Move, Outcome};

pub struct Round(Move, Move);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseRoundError;

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(' ').ok_or(ParseRoundError)?;
        let (m1, m2) = (
            Move::from_str(p1).map_err(|_| ParseRoundError)?,
            Move::from_str(p2).map_err(|_| ParseRoundError)?,
        );
        Ok(Round(m1, m2))
    }
}

impl Round {
    pub fn score(&self) -> usize {
        let move_score = match self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        let outcome_score = match self.1.vs(&self.0) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };
        move_score + outcome_score
    }
}
