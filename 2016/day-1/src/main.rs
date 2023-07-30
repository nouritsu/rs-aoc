mod character;
mod instruction;

use character::Character;
use common::functions::{get_input_file_arg, read_input_str};
use instruction::Instruction;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input_str(&f)?;

    let mut character = Character::new();
    input
        .split(", ")
        .map(|s| Instruction::from_str(s).expect("invalid input"))
        .for_each(|i| character.apply(&i));
    let output = character.final_pos();

    println!("Final distance: {}", output);

    Ok(())
}
