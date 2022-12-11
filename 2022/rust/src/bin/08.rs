#![allow(clippy::indexing_slicing)]
// yes i'm careful

use anyhow::{Context, Result};
use aoc::open;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

struct Tree {
    visible: bool,
    height: usize,
    score: usize,
}

fn parse(input: &str) -> Result<Vec<Vec<Tree>>> {
    let mut v = Vec::new();
    for line in input.trim().lines() {
        let mut v2 = Vec::new();
        for c in line.chars() {
            v2.push(Tree {
                visible: false,
                height: c.to_digit(10).context("Incorrect input")?.try_into()?,
                score: 1,
            });
        }
        v.push(v2);
    }
    Ok(v)
}

trait Processor {
    fn new() -> Self;
    fn reset(&mut self);
    fn process(&mut self, t: &mut Tree);
}

struct VisProc(Option<usize>);

impl Processor for VisProc {
    fn new() -> Self {
        Self(None)
    }

    fn reset(&mut self) {
        *self = Self(None);
    }

    fn process(&mut self, t: &mut Tree) {
        if !matches!(self.0, Some(ph) if t.height <= ph) {
            t.visible = true;
            self.0 = Some(t.height);
        }
    }
}

struct ScoreProc([usize; 10]);

impl Processor for ScoreProc {
    fn new() -> Self {
        Self([0; 10])
    }

    fn reset(&mut self) {
        self.0.fill(0);
    }

    fn process(&mut self, t: &mut Tree) {
        t.score *= self.0[t.height];
        for (i, h) in self.0.iter_mut().enumerate() {
            if i <= t.height {
                *h = 1;
            } else {
                *h += 1;
            }
        }
    }
}

fn process<P: Processor>(v: &mut [Vec<Tree>]) {
    let mut p_x = P::new();
    let mut p_rev_x = P::new();
    let mut p_y = P::new();
    let mut p_rev_y = P::new();
    let col_len = v.len();
    for i in 0..col_len {
        p_x.reset();
        p_rev_x.reset();
        p_y.reset();
        p_rev_y.reset();
        let row_len = v[i].len();
        for j in 0..row_len {
            p_x.process(&mut v[i][j]);
            p_rev_x.process(&mut v[i][row_len - 1 - j]);
            p_y.process(&mut v[j][i]);
            p_rev_y.process(&mut v[col_len - 1 - j][i]);
        }
    }
}

fn part1(f: File) -> Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;
    let mut t = parse(&input)?;
    process::<VisProc>(&mut t);
    Ok(t.into_iter()
        .flatten()
        .map(|t| usize::from(t.visible))
        .sum())
}

fn part2(f: File) -> Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;
    let mut t = parse(&input)?;

    process::<ScoreProc>(&mut t);
    t.into_iter()
        .flatten()
        .map(|t| t.score)
        .max()
        .context("No max??")
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1(open!("input.txt")?)?);
    println!("Part 2: {}", part2(open!("input.txt")?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 21);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 1801);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 8);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 209880);
    }
}
