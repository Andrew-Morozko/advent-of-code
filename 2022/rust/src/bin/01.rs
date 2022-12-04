use anyhow::{Context, Result};
use aoc::open;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn parse(f: File, mut process: impl FnMut(u64)) -> Result<()> {
    let mut sum = 0;
    let r = BufReader::new(f);

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

fn part1(f: File) -> Result<u64> {
    let max = &mut None;
    parse(f, |cur_sum| {
        if !matches!(max, Some(sum) if cur_sum <= *sum) {
            *max = Some(cur_sum);
        }
    })?;

    max.context("No elves in data")
}

fn part2(f: File) -> Result<u64> {
    let max = &mut [None; 3];
    parse(f, |mut sum| {
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
    println!("Max calories: {}", part1(open!("input.txt")?)?);
    println!("Sum of top 3 max cals: {}", part2(open!("input.txt")?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 24000);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 72478);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 45000);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 210367);
    }
}
