use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn get_input_file_arg() -> String {
    let args = env::args().collect::<Vec<String>>();
    args.get(1)
        .expect("pass in input file path thorugh command line")
        .to_string()
}

pub fn read_input(p: &impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(p)?).lines().collect()
}

pub fn read_input_str(p: &impl AsRef<Path>) -> io::Result<String> {
    io::read_to_string(File::open(p)?)
}
