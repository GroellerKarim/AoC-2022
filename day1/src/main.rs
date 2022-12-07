use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    // First puzzle
    if let Ok(lines) = read_lines("./input1.txt") {
        let highest_calories = calorie_counting(lines);
        println!("{}", highest_calories)
    }
}

pub fn calorie_counting(lines: Lines<BufReader<File>>) -> String {
    let mut elf_vec: Vec<Elf> = Vec::new();
    let mut calories = 0;
    let mut current_line = 1;

    // Convert the String into useful data
    for line_result in lines {
        if let Ok(line) = line_result {
            if line.is_empty() {
                elf_vec.push(Elf { calories });
                calories = 0;
            } else {
                let parsed_number_result = line.parse::<i32>();

                let parsed_number: i32;

                match parsed_number_result {
                    Ok(ok) => parsed_number = ok,
                    Err(_) => {
                        return format!("{line}(line: {current_line}) is not a number!");
                    }
                }

                calories += parsed_number;
            }
        }
        current_line = current_line + 1;
    }

    // Get the elf with the highest calories
    let mut highest_elf: Elf = Elf { calories: 0 };

    for elf in elf_vec {
        if elf.calories > highest_elf.calories {
            highest_elf = elf;
        }
    }

    return format!("Highest Elf -> {}", highest_elf.calories);
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
