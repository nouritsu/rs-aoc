mod r#move;
mod round;

use common::functions::{get_input_file_arg, read_input};
use round::Round;
use std::{io, str::FromStr};

use crate::r#move::Move;

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let output_1 = round_1(&input);
    let output_2 = round_2(&input);

    println!("Total Score (Part 1): {}", output_1);
    println!("Total Score (Part 2): {}", output_2);

    Ok(())
}

fn round_1(vec: &Vec<String>) -> usize {
    vec.iter()
        .map(|s| Round::from_str(s).expect("invalid input").score())
        .sum()
}

fn round_2(vec: &Vec<String>) -> usize {
    vec.iter()
        .map(|s| {
            let m1 = Move::from_str(&s.chars().nth(0).expect("invalid input").to_string())
                .expect("invalid input");
            let m2 = match s.chars().nth(2).expect("invalid input") {
                'X' => m1.defeats(),
                'Y' => m1.clone(),
                'Z' => m1.defeated(),
                _ => panic!("invalid input"),
            };
            Round::new(m1, m2)
        })
        .map(|round| round.score())
        .sum()
}
