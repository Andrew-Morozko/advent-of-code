use anyhow::{bail, Context, Result};
use aoc::open;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

trait Score {
    fn score(&self) -> u64;
}

impl Score for Move {
    fn score(&self) -> u64 {
        match self {
            // score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => bail!("Incorrect move: '{value}'"),
        })
    }
}

impl From<(Move, Outcome)> for Move {
    fn from(p: (Move, Outcome)) -> Self {
        let (opp, result) = p;
        match result {
            Outcome::Loss => match opp {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Draw => opp,
            Outcome::Win => match opp {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Score for Outcome {
    fn score(&self) -> u64 {
        match self {
            // score for the outcome of the round
            // (0 if you lost, 3 if the round was a draw, and 6 if you won).
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
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

impl TryFrom<&str> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => bail!("Incorrect outcome: '{value}'"),
        })
    }
}

fn score_tournament(parse_game: impl Fn(&str, &str) -> Result<(Move, Outcome)>) -> Result<u64> {
    let r = BufReader::new(open("input.txt")?);

    let mut sum = 0;

    for l in r.lines() {
        let l = l.context("Failed to read line")?;
        let l = l.trim();
        if l.is_empty() {
            continue;
        }
        let (a, b) = l.split_once(' ').context("Failed to split the line")?;
        let (my_move, outcome) = parse_game(a, b)?;
        sum += my_move.score() + outcome.score();
    }
    Ok(sum)
}

fn part1() -> Result<u64> {
    score_tournament(|opp: &str, me: &str| {
        let (opp, me) = (Move::try_from(opp)?, Move::try_from(me)?);
        Ok((me, Outcome::from((me, opp))))
    })
}

fn part2() -> Result<u64> {
    score_tournament(|opp: &str, outcome: &str| {
        let (opp, outcome) = (Move::try_from(opp)?, Outcome::try_from(outcome)?);
        let me = Move::from((opp, outcome));
        Ok((me, outcome))
    })
}

fn main() -> Result<()> {
    println!("Score pt1: {}", part1()?);
    println!("Score pt2: {}", part2()?);
    Ok(())
}
