use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct MoveCrate {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct ParseMoveError;

impl FromStr for MoveCrate {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(' ')
            .filter(|s| s.chars().all(|c| c.is_numeric()))
            .map(|s| s.parse::<usize>().map_err(|_| ParseMoveError))
            .collect::<Result<Vec<usize>, ParseMoveError>>()?;

        if numbers.len() != 3 {
            return Err(ParseMoveError);
        }

        Ok(MoveCrate::new(numbers[0], numbers[1] - 1, numbers[2] - 1))
    }
}

impl MoveCrate {
    pub fn new(count: usize, from: usize, to: usize) -> Self {
        MoveCrate { count, from, to }
    }

    pub fn apply_pop(&self, v: &mut Vec<Vec<char>>) {
        for _ in 0..self.count {
            let e = v[self.from].pop().expect("invalid input");
            v[self.to].push(e);
        }
    }

    pub fn apply_chunk(&self, v: &mut Vec<Vec<char>>) {
        let mut chunk = Vec::new();

        for _ in 0..self.count {
            chunk.push(v[self.from].pop().expect("invalid input"))
        }

        chunk.reverse();

        for c in chunk {
            v[self.to].push(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let m = MoveCrate::from_str("move 2 from 5 to 9").unwrap();
        assert_eq!(MoveCrate::new(2, 4, 8), m);
    }
}
