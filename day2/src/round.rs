pub struct Round {
    opponent: char,
    myself: char,
}

// A = Rock, B = Paper, C = Scissors
// X = Rock - 1, Y = Paper -2 , Z = Scissors -3
// Draw 3, won 6, loss 0

impl Round {
    pub fn calculate_score(&self) -> i32 {
        let opponent = self.opponent;
        let myself = self.myself;
        if opponent == 'A' {
            if myself == 'X' {
                return 4
            }
            else if myself == 'Y' {
                return 8
            }
            else if myself == 'Z' {
                return 3
            }
        }
        else if opponent == 'B' {
            if myself == 'X' {
                return 1
            }
            else if myself == 'Y' {
                return 5
            }
            else if myself == 'Z' {
                return 9
            }
        }
        else if opponent == 'C' {
            if myself == 'X' {
                return 7
            }
            else if myself == 'Y' {
                return 2
            }
            else if myself == 'Z' {
                return 6
            }
        }
        else {
            panic!("{}", format!("Should not happen, opponent was weird char { }", opponent))
        }
        0
    }
}