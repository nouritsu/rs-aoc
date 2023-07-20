use common::functions::{get_file_path, read_input};
use std::io;

fn main() -> io::Result<()> {
    let f = get_file_path();

    let mut vec = read_input(&f)?
        .split(|x| x.is_empty())
        .map(|e| e.iter().map(|s| s.parse::<usize>().unwrap()).sum::<usize>())
        .collect::<Vec<usize>>();
    vec.sort_unstable_by(|a, b| b.cmp(a));

    let top = vec[0];
    let top_3 = vec[0..3].iter().sum::<usize>();

    println!("Top           = {}", top);
    println!("Sum of Top 3  = {}", top_3);

    Ok(())
}
