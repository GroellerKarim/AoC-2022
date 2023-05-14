use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {

    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_count = 0;
        
        for line_result in lines {
            if let Ok(line) = line_result {
                priority_count += calculate_line(line);
            }
        }

        println!("{}", priority_count);
    }

}

fn calculate_line(line: String) -> u32 {
    let mut last_char: char = '|';
    for char in line.chars() {
        if char == last_char { break };
        last_char = char;
    }
    println!("{}", last_char);
    let num: u32 = 31;
    let mut priority: u32 = last_char.to_digit(2).unwrap() & num;

    if last_char.is_uppercase() { priority += 26 }

    return priority;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
