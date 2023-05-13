use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    ops::Index,
    path::Path,
};

mod round;

fn main() {
    println!("Hello, world!");
    // Rock Paper Scissors

    if let Ok(lines) = read_lines("./input.txt") {

    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
