mod round;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};
use crate::round::Round;


fn main() {

    // Rock Paper Scissors
    if let Ok(lines) = read_lines("./input.txt") {
        let mut score_part_1 = 0;
        let mut score_part_2 = 0;
        for line_result in lines {
            if let Ok(line) = line_result {
                part_one(&mut score_part_1, line.clone());
                part_two(&mut score_part_2, line.clone());
            }
        }
        println!("part 1: {}, part 2: {}", score_part_1, score_part_2);
    }
}

fn part_one(score_part_1: &mut i32, line: String) {
    let mut split_line = line.split(' ');
    let round = Round {
        opponent: split_line.next().unwrap_or("").parse().unwrap(),
        myself: split_line.next().unwrap_or("").parse().unwrap()
    };

    *score_part_1 +=  round.calculate_score();
}

// A = Rock, B = Paper, C = Scissors
// X = Rock - 1, Y = Paper -2 , Z = Scissors -3
// Draw 3, won 6, loss 0
fn part_two(score_part_2: &mut i32, line: String) {
    let mut split_line = line.split(' ');

    let round = create_needed_scenario(
        split_line.next().unwrap_or("").parse().unwrap(),
        split_line.next().unwrap_or("").parse().unwrap());

    *score_part_2 +=  round.calculate_score();
}

fn create_needed_scenario(opponent: char, myself: char) -> Round {
    let mut needed_char_for_scenario: char = ' ';
    if opponent == 'A' {
        if myself == 'X' {
            needed_char_for_scenario = 'Z'
        }
        else if myself == 'Y' {
            needed_char_for_scenario = 'X'
        }
        else if myself == 'Z' {
            needed_char_for_scenario = 'Y'
        }
    }
    else if opponent == 'B' {
        if myself == 'X' {
            needed_char_for_scenario = 'X'
        }
        else if myself == 'Y' {
            needed_char_for_scenario = 'Y'
        }
        else if myself == 'Z' {
            needed_char_for_scenario = 'Z'
        }
    }
    else if opponent == 'C' {
        if myself == 'X' {
            needed_char_for_scenario = 'Y'
        }
        else if myself == 'Y' {
            needed_char_for_scenario = 'Z'
        }
        else if myself == 'Z' {
            needed_char_for_scenario = 'X'
        }
    }
    else {
        panic!("{}", format!("Should not happen, opponent was weird char { }", opponent))
    }

    Round {
        opponent,
        myself: needed_char_for_scenario
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
