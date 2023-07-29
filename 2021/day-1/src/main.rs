use std::io;

use common::functions::{get_input_file_arg, read_input};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?
        .iter()
        .map(|num| num.parse().expect("invalid input"))
        .collect::<Vec<usize>>();

    let output = input.windows(2).map(|a| a[1] > a[0]).filter(|b| *b).count();

    println!(
        "Number of measurements larger than the previous: {}",
        output
    );

    Ok(())
}
