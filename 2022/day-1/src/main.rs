use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let f = args.get(1).expect("pass input file in command line");

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

fn read_input(p: &impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(p)?).lines().collect()
}
