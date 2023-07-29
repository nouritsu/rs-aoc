use common::functions::{get_input_file_arg, read_input_str};
use std::io;

fn main() -> io::Result<()> {
    let f = get_input_file_arg();
    let input = read_input_str(&f)?;

    let output1 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid input"),
        })
        .sum::<isize>();

    let mut output2 = 0usize;
    let mut sum = 0isize;
    for (i, e) in input.chars().enumerate() {
        match e {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("invalid input"),
        }
        if sum == -1 {
            output2 = i + 1;
            break;
        }
    }

    println!("Resultant floor: {}", output1);
    println!("First index at which floor is -1: {}", output2);

    Ok(())
}
