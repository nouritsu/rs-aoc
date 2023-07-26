use crate::range::Range;
use std::str::FromStr;

pub struct Pair(Range, Range);

#[derive(Debug)]
pub struct ParsePairError;

impl FromStr for Pair {
    type Err = ParsePairError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rastr, rbstr) = s.split_once(',').ok_or(ParsePairError)?;
        let ra = Range::from_str(rastr).map_err(|_| ParsePairError)?;
        let rb = Range::from_str(rbstr).map_err(|_| ParsePairError)?;

        Ok(Pair::new(ra, rb))
    }
}

impl Pair {
    pub fn new(ra: Range, rb: Range) -> Self {
        Pair(ra, rb)
    }

    pub fn fully_contains(&self) -> bool {
        self.0.superset(&self.1) || self.1.superset(&self.0)
    }
}
