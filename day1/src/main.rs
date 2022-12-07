use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    ops::Index,
    path::Path,
};

fn main() {
    // First puzzle
    if let Ok(lines) = read_lines("./input1.txt") {
        match calorie_counting(lines) {
            Ok(mut elves) => {
                let highest_calories = highest_elf(&elves);
                let top_three_calories = top_three_elves(&mut elves);
                println!("Hightest Calorie Elf {}", highest_calories);
                println!("Top Three Elves: {}", top_three_calories);
            }
            Err(e) => println!("{}", e),
        }
    }
}

pub fn highest_elf(elf_vec: &Vec<i32>) -> String {
    // Get the elf with the highest calories
    let mut highest_elf = 0;

    for elf in elf_vec {
        if elf > &highest_elf {
            highest_elf = *elf;
        }
    }

    return format!("Highest Elf -> {}", highest_elf);
}

pub fn top_three_elves(elf_vec: &mut Vec<i32>) -> String {
    if elf_vec.len() < 3 {
        return "There are less than 3 elves on this journey".to_string();
    }

    elf_vec.sort_by(|a, b| b.cmp(a));

    let total_calories = elf_vec.index(0) + elf_vec.index(1) + elf_vec.index(2);
    return total_calories.to_string();
}

pub fn calorie_counting(lines: Lines<BufReader<File>>) -> Result<Vec<i32>, String> {
    let mut elf_vec: Vec<i32> = Vec::new();
    let mut calories = 0;
    let mut current_line = 1;

    // Convert the String into useful data
    for line_result in lines {
        if let Ok(line) = line_result {
            if line.is_empty() {
                elf_vec.push(calories);
                calories = 0;
            } else {
                let parsed_number_result = line.parse::<i32>();

                let parsed_number: i32;

                match parsed_number_result {
                    Ok(ok) => parsed_number = ok,
                    Err(_) => {
                        return Err(format!("{line}(line: {current_line}) is not a number!"));
                    }
                }

                calories += parsed_number;
            }
        }
        current_line = current_line + 1;
    }

    Ok(elf_vec)
}

pub struct Elf {
    pub calories: i32,
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
