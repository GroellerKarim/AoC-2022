use std::{
    fs::File,
    io::{self, BufRead},
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

    let mid = line.chars().count() / 2;
    let _first_compartment = &line[..mid];
    let _second_compartment= &line[mid..];

    println!("line {}", line);
    println!("first comp {}, second comp {}", _first_compartment, _second_compartment);

    let mut last_char: char = '|';
    
    'outer: for char in _first_compartment.chars() {
        last_char = char;
        for char2 in _second_compartment.chars() {
            if char2 == last_char { break 'outer; }
        }
    }

    let num: u32 = 31; 
    let mut priority: u32 = (last_char as u32) & num;

    if last_char.is_uppercase() { priority += 26 }

    println!("priority for char {} is {}", last_char, priority);
    return priority;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
