use crate::instruction::{Direction, Instruction};

pub struct Character {
    direction: Facing,
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

impl Character {
    pub fn new() -> Self {
        Character {
            direction: Facing::North,
            x: 0,
            y: 0,
        }
    }

    pub fn apply(&mut self, i: &Instruction) {
        self.direction = match i.0 {
            Direction::Left => self.direction.left(),
            Direction::Right => self.direction.right(),
        };

        match self.direction {
            Facing::North => self.y += i.1 as isize,
            Facing::South => self.y -= i.1 as isize,
            Facing::East => self.x += i.1 as isize,
            Facing::West => self.x -= i.1 as isize,
        }
    }

    pub fn final_pos(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Facing {
    pub fn left(&self) -> Self {
        match &self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }

    pub fn right(&self) -> Self {
        match &self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }
}
