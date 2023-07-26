mod pair;
mod range;

use common::functions::{get_input_file_arg, read_input};
use std::{io, str::FromStr};

use crate::pair::Pair;

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let output = input
        .iter()
        .map(|s| Pair::from_str(s).expect("invalid input").fully_contains())
        .filter(|b| *b)
        .count();

    println!("Fully containing ranges in pairs: {}", output);

    Ok(())
}
