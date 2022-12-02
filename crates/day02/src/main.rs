use anyhow::Error;
use std::str::FromStr;

pub const PART1: bool = true;

fn split2(s: &str) -> Result<(&str, &str), Error> {
    let mut splitter = s.split_ascii_whitespace();
    let a = splitter.next().ok_or_else(|| Error::msg("Missing token"))?;
    let b = splitter.next().ok_or_else(|| Error::msg("Missing token"))?;
    Ok((a, b))
}

// Shape

#[derive(Clone, Copy, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn wins_against(self) -> Shape {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(self) -> Shape {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(Error::msg("Invalid Shape")),
        }
    }
}

// Outcome

#[derive(Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn new(theirs: Shape, ours: Shape) -> Self {
        if ours.wins_against() == theirs {
            Self::Win
        } else if ours.loses_against() == theirs {
            Self::Loss
        } else {
            Self::Draw
        }
    }

    fn score(self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(Error::msg("Invalid Outcome")),
        }
    }
}

fn main() {
    let score = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (token1, token2) = split2(&line).unwrap();
            let theirs = Shape::from_str(token1).unwrap();
            let ours = if PART1 {
                Shape::from_str(token2).unwrap()
            } else {
                let outcome = Outcome::from_str(token2).unwrap();
                match outcome {
                    Outcome::Loss => theirs.wins_against(),
                    Outcome::Draw => theirs,
                    Outcome::Win => theirs.loses_against(),
                }
            };

            Outcome::new(theirs, ours).score() + ours.score()
        })
        .sum::<u32>();

    println!("{score}");
}
