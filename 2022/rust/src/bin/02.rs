use anyhow::{bail, Context, Result};
use aoc::open;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

trait Score {
    fn score(self) -> u64;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u64)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Score for Move {
    fn score(self) -> u64 {
        #![allow(clippy::as_conversions)]
        self as u64
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => bail!("Incorrect move: '{s}'"),
        })
    }
}

impl From<(Self, Outcome)> for Move {
    fn from(p: (Self, Outcome)) -> Self {
        let (opp, result) = p;
        match result {
            Outcome::Loss => match opp {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
            Outcome::Draw => opp,
            Outcome::Win => match opp {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u64)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Score for Outcome {
    fn score(self) -> u64 {
        #![allow(clippy::as_conversions)]
        self as u64
    }
}

impl From<(Move, Move)> for Outcome {
    fn from(p: (Move, Move)) -> Self {
        match p {
            (a, b) if a == b => Self::Draw,
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => Self::Win,
            _ => Self::Loss,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => bail!("Incorrect outcome: '{s}'"),
        })
    }
}

fn score_tournament(
    f: File,
    parse_game: impl Fn(&str, &str) -> Result<(Move, Outcome)>,
) -> Result<u64> {
    let reader = BufReader::new(f);

    let mut sum = 0;

    for l in reader.lines() {
        let line = l.context("Failed to read line")?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (a, b) = line.split_once(' ').context("Failed to split the line")?;
        let (my_move, outcome) = parse_game(a, b)?;
        sum += my_move.score() + outcome.score();
    }
    Ok(sum)
}

fn part1(f: File) -> Result<u64> {
    score_tournament(f, |opp: &str, me: &str| {
        let (opp, me) = (opp.parse::<Move>()?, me.parse::<Move>()?);
        Ok((me, Outcome::from((me, opp))))
    })
}

fn part2(f: File) -> Result<u64> {
    score_tournament(f, |opp: &str, outcome: &str| {
        let (opp, outcome) = (opp.parse::<Move>()?, outcome.parse::<Outcome>()?);
        let me = Move::from((opp, outcome));
        Ok((me, outcome))
    })
}

fn main() -> Result<()> {
    println!("Score pt1: {}", part1(open!("input.txt")?)?);
    println!("Score pt2: {}", part2(open!("input.txt")?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 15);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 15523);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 12);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 15702);
    }
}
