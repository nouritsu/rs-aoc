mod pair;
mod range;

use common::functions::{get_input_file_arg, read_input};
use std::io;

use crate::pair::Pair;

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let output1 = input
        .iter()
        .map(|s| s.parse::<Pair>().expect("invalid input").fully_contains())
        .filter(|b| *b)
        .count();

    let output2 = input
        .iter()
        .map(|s| s.parse::<Pair>().expect("invalid input").overlap())
        .filter(|b| *b)
        .count();

    println!("Fully containing ranges in pairs: {}", output1);
    println!("Some overlap in pairs: {}", output2);

    Ok(())
}
