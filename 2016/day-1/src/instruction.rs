use std::str::FromStr;

pub struct Instruction(pub Direction, pub usize);

#[derive(Debug)]
pub struct ParseInstructionError;

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct ParseDirectionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dis_str) = s.split_at(1);
        let dir = dir_str.parse().map_err(|_| ParseInstructionError)?;
        let dis = dis_str.parse().map_err(|_| ParseInstructionError)?;

        Ok(Self::new(dir, dis))
    }
}

impl Instruction {
    pub fn new(dir: Direction, dis: usize) -> Self {
        Instruction(dir, dis)
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseDirectionError),
        })
    }
}
