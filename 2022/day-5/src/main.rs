mod r#move;

use common::functions::{get_input_file_arg, read_input};
use r#move::MoveCrate;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?;

    let crates = vec![
        vec!['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
        vec!['N', 'V', 'G', 'P', 'H', 'W', 'B'],
        vec!['F', 'W', 'B', 'J', 'G'],
        vec!['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'],
        vec!['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'],
        vec!['B', 'C', 'W', 'G', 'F', 'S'],
        vec!['H', 'T', 'P', 'M', 'Q', 'B', 'W'],
        vec!['F', 'S', 'W', 'T'],
        vec!['N', 'C', 'R'],
    ];

    let instructions = input
        .iter()
        .map(|s| MoveCrate::from_str(s).expect("invalid input"))
        .collect::<Vec<MoveCrate>>();

    let mut crates_mut = crates.clone();
    instructions
        .iter()
        .for_each(|i| i.apply_pop(&mut crates_mut));
    let output1 = crates_mut
        .iter()
        .map(|v| v.last().expect("invalid input").to_owned())
        .collect::<String>();

    let mut crates_mut = crates.clone();
    instructions
        .iter()
        .for_each(|i| i.apply_chunk(&mut crates_mut));
    let output2 = crates_mut
        .iter()
        .map(|v| v.last().expect("invalid input").to_owned())
        .collect::<String>();

    println!("Tops of crates with CrateMover 9000: {}", output1);
    println!("Tops of crates with CrateMover 9001: {}", output2);

    Ok(())
}
