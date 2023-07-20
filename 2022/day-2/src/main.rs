mod r#move;
mod round;

use common::functions::{get_input_file_arg, read_input};
use round::Round;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let output = input
        .iter()
        .map(|s| Round::from_str(s).expect("invalid input").score())
        .sum::<usize>();

    println!("Total Score: {}", output);

    Ok(())
}
