use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, vec, collections::HashSet,
};

fn main() {

    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_count = 0;

        // Part 2
        let mut priority_count_part2 = 0;
        let mut three_group_elves_lines: Vec<String> = vec![];
        
        for line_result in lines {
            if let Ok(line) = line_result {
                priority_count += calculate_line(line.clone());

                three_group_elves_lines.push(line.clone());
                if three_group_elves_lines.len() == 3 {
                    priority_count_part2 = calculate_group(three_group_elves_lines);

                    // Reset the items in group
                    three_group_elves_lines = vec![];
                }
            }
        }

        println!("{}", priority_count);
        println!("Part2 {}", priority_count_part2);
    }

}

fn calculate_group(groups: Vec<String>) -> u32 {
    let group1: HashSet<char> = groups[0].chars().collect();
    let group2: HashSet<char>= groups[1].chars().collect();
    let group3: HashSet<char> = groups[2].chars().collect();

    let common: HashSet<char> = group1.intersection(&group2).cloned().collect();
    let common: HashSet<char> = common.intersection(&group3).cloned().collect();

    return calculate_priority(*common.iter().next().unwrap());
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

    return calculate_priority(last_char);
}

fn calculate_priority(char: char) -> u32 {
    let mut priority: u32 = (char as u32) & 31;
    if char.is_uppercase() { priority += 26 }
    println!("priority for char {} is {}", char, priority);
    return priority;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
