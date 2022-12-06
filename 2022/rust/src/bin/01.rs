use anyhow::{Context, Result};
use aoc::{extra_itertools::ExtraItertools, open};
use itertools::Itertools;
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

enum Val {
    Val(u64),
    CalcSubSum,
}

// I WANTED to go full iterator on this problem. I succeeded (BUT AT WHAT COST)
fn part1_full_iter(f: File) -> Result<u64> {
    // Temp values (are moved into closures)
    let mut was_last_empty = true;
    let mut local_sum = 0;

    // Read line by line
    BufReader::new(f)
        .lines()
        // Convert io error to anyhow
        .map(|e| e.context("Failed to read the line"))
        // if no errors - attempt to parse string into ether u64
        // or an empty line (signals to calculate the partial sum for one elf)
        .map_ok_res(move |line| {
            let line = line.trim();
            if line.is_empty() {
                Ok(Val::CalcSubSum)
            } else {
                line.parse().context("Failed to parse").map(Val::Val)
            }
        })
        // Ensure that even if file doesn't end in empty line we're still calculating partial sum
        // for the last elf
        .chain([Ok(Val::CalcSubSum)])
        // Filter:
        // empty lines at the start of the file (was_last_empty is initially set to true)
        // multiple consecutive empty lines (lets only one Val::CalcSubSum through)
        .filter_ok(|v| match v {
            Val::CalcSubSum => {
                if was_last_empty {
                    false
                } else {
                    was_last_empty = true;
                    true
                }
            }
            Val::Val(_) => {
                was_last_empty = false;
                true
            }
        })
        // sums consecutive Val::Val into local_sum and returns it when CalcSubSum is encountared
        .filter_map_ok(move |v| match v {
            Val::Val(v) => {
                local_sum += v;
                None
            }
            Val::CalcSubSum => {
                let r = Some(local_sum);
                local_sum = 0;
                r
            }
        })
        // folds sub-sums (returns the largest, or None if the file was compeltely empty)
        .fold_ok(None, |max, v| {
            max.map_or(Some(v), |max_v| if max_v < v { Some(v) } else { max })
        })?
        .context("Empty input")
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
    println!(
        "Max calories (iter): {}",
        part1_full_iter(open!("input.txt")?)?
    );
    println!("Sum of top 3 max cals: {}", part2(open!("input.txt")?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 24000);
        assert_eq!(part1_full_iter(open!("test.txt").unwrap()).unwrap(), 24000);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 72478);
        assert_eq!(part1_full_iter(open!("input.txt").unwrap()).unwrap(), 72478);
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
