use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputfile = File::open(&args[1]).expect("Cannot open file.");
    let reader = BufReader::new(inputfile);
    println!(
        "Solution: {}",
        solve_2(reader.lines().map(|l| l.expect("Cannot read file.")))
    )
}

fn round(score: u64, line: &str) -> u64 {
    score + compute_score(line)
}

fn compute_score(line: &str) -> u64 {
    let values: Vec<&str> = line.split(" ").collect();
    let opponent = Move::from_opponent(values[0]);
    let player = Move::from_player(values[1]);
    player.battle(&opponent) + player.score()
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Move::Rock => "rock",
            Move::Paper => "paper",
            Move::Scissor => "scissor",
        };
        write!(f, "{}", s)
    }
}

impl Move {
    fn from_opponent(line: &str) -> Self {
        match line {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            _ => panic!("Impossible!"),
        }
    }

    fn from_player(line: &str) -> Self {
        match line {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => panic!("Impossible!"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
    fn battle(&self, other: &Self) -> u64 {
        match (self, other) {
            (Move::Rock, Move::Rock) => 3,
            (Move::Rock, Move::Paper) => 0,
            (Move::Rock, Move::Scissor) => 6,
            (Move::Paper, Move::Rock) => 6,
            (Move::Paper, Move::Paper) => 3,
            (Move::Paper, Move::Scissor) => 0,
            (Move::Scissor, Move::Rock) => 0,
            (Move::Scissor, Move::Paper) => 6,
            (Move::Scissor, Move::Scissor) => 3,
        }
    }

    fn lose(&self) -> Move {
        match self {
            Move::Rock => Self::Scissor,
            Move::Paper => Self::Rock,
            Move::Scissor => Self::Paper,
        }
    }

    fn beat(&self) -> Move {
        match self {
            Move::Rock => Self::Paper,
            Move::Paper => Self::Scissor,
            Move::Scissor => Self::Rock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Outcome::Win => "win",
            Outcome::Lose => "lose",
            Outcome::Draw => "draw",
        };
        write!(f, "{}", s)
    }
}

impl Outcome {
    fn from_string(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Impossible!"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn to_move(&self, opponent: Move) -> Move {
        match self {
            Outcome::Win => opponent.beat(),
            Outcome::Lose => opponent.lose(),
            Outcome::Draw => opponent,
        }
    }
}

fn solve_1<T>(input: T) -> u64
where
    T: Iterator<Item = String>,
{
    input.fold(0, |score, line| round(score, &line))
}

fn solve_2<T>(input: T) -> u64
where
    T: Iterator<Item = String>,
{
    input.fold(0, |score, line| round_2(score, &line))
}

fn round_2(score: u64, line: &str) -> u64 {
    score + compute_score_2(line)
}

fn compute_score_2(line: &str) -> u64 {
    let values: Vec<&str> = line.split(" ").collect();
    let opponent = Move::from_opponent(values[0]);
    let outcome = Outcome::from_string(values[1]);
    let player = outcome.to_move(opponent);
    let out = player.score() + outcome.score();
    println!("{}, Opponent plays {} and outcome is a {} so I play {} for {} points", line, opponent, outcome, player, out);
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = r#"A Y
B X
C Z
"#;
        assert_eq!(solve_1(input.lines().map(|r| r.to_string())), 15);
    }

    #[test]
    fn example_2() {
        let input = r#"A Y
B X
C Z
"#;
        assert_eq!(solve_2(input.lines().map(|r| r.to_string())), 12);
    }
}
