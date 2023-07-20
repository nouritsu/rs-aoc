use common::functions::{get_input_file_arg, read_input};
use std::io;

fn main() -> io::Result<()> {
    let f = get_input_file_arg();

    println!("{:?}", read_input(&f)?);
    Ok(())
}
