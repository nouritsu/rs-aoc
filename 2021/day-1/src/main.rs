use std::io;

use common::functions::{get_input_file_arg, read_input};

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input(&f)?
        .iter()
        .map(|num| num.parse().expect("invalid input"))
        .collect::<Vec<usize>>();

    let output1 = input.windows(2).filter(|a| a[1] > a[0]).count();

    let output2 = input
        .windows(3)
        .map(|a| a.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|a| a[1] > a[0])
        .count();

    println!(
        "Number of measurements larger than the previous: {}",
        output1
    );

    println!(
        "Number of measurements larger than the previous (Sum with windows of 3): {}",
        output2
    );

    Ok(())
}
