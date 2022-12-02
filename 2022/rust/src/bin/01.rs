use anyhow::{Context, Result};
use aoc::open;
use std::io::{prelude::*, BufReader};

fn parse(mut process: impl FnMut(u64)) -> Result<()> {
    let r = BufReader::new(open("input.txt")?);

    let mut sum = 0;

    for l in r.lines().chain(Some(Ok(String::new()))) {
        let l = l.context("Failed to read line")?;
        let l = l.trim();
        if !l.is_empty() {
            sum += l.parse::<u64>().context("Failed to parse non-empty line")?;
            continue;
        }
        process(sum);
        sum = 0;
    }
    Ok(())
}

fn part1() -> Result<u64> {
    let max = &mut None;
    parse(|cur_sum| {
        if !matches!(max, Some(sum) if cur_sum <= *sum) {
            *max = Some(cur_sum);
        }
    })?;

    max.context("No elves in data")
}

fn part2() -> Result<u64> {
    let max = &mut [None; 3];
    parse(|mut sum| {
        for el in max.iter_mut() {
            match el {
                Some(val) if *val < sum => {
                    std::mem::swap(val, &mut sum);
                }
                None => {
                    *el = Some(sum);
                    break;
                }
                _ => {}
            }
        }
    })?;

    max.iter()
        .try_fold(0, |sum, el| el.map(|el| sum + el))
        .context("Less than 3 elves in data")
}

fn main() -> Result<()> {
    println!("Max calories: {}", part1()?);
    println!("Sum of top 3 max cals: {}", part2()?);
    Ok(())
}
