use std::str::FromStr;

pub struct Range(usize, usize);

pub struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (astr, bstr) = s.split_once('-').ok_or(ParseRangeError)?;
        let a = astr.parse::<usize>().map_err(|_| ParseRangeError)?;
        let b = bstr.parse::<usize>().map_err(|_| ParseRangeError)?;

        Ok(Range::new(a, b))
    }
}

impl Range {
    pub fn new(a: usize, b: usize) -> Self {
        Range(a, b)
    }

    pub fn superset(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}
