use anyhow::{anyhow, bail, Context, Result};
use aoc::open;
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};
use trim_in_place::TrimInPlace;

fn gen_priority_map() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<_, _>>()
}

fn part1(f: File) -> Result<usize> {
    let map = gen_priority_map();
    let mut sum = 0;

    'next_line: for line in BufReader::new(f).lines() {
        let mut line = line.context("Failed to read line")?;
        line.trim_in_place();
        if line.is_empty() {
            continue;
        }
        if line.len() % 2 != 0 {
            bail!("Odd number of items in line");
        }
        let (l, r) = line.split_at(line.len() / 2);
        let mut items: u64 = 0;
        for c in l.chars() {
            items |= 1 << map.get(&c).with_context(|| anyhow!("Incorrect item {c}"))?;
        }
        for c in r.chars() {
            let prio = *map.get(&c).with_context(|| anyhow!("Incorrect item {c}"))?;
            if (items & 1 << prio) != 0 {
                sum += prio;
                continue 'next_line;
            }
        }
        bail!("No matches in line {line}");
    }
    Ok(sum)
}

fn part2(f: File) -> Result<usize> {
    let map = gen_priority_map();
    let mut sum = 0;
    let mut line_no: u32 = 1;
    let mut items_in_3_lines = u64::MAX;

    'next_line: for line in BufReader::new(f).lines() {
        let mut line = line.context("Failed to read line")?;
        line.trim_in_place();
        if line.is_empty() {
            continue;
        }
        if line_no == 3 {
            for c in line.chars() {
                let prio = *map.get(&c).with_context(|| anyhow!("Incorrect item {c}"))?;
                if (items_in_3_lines & 1 << prio) != 0 {
                    sum += prio;
                    line_no = 1;
                    items_in_3_lines = u64::MAX;
                    continue 'next_line;
                }
            }
        } else {
            let mut items = 0;
            for c in line.chars() {
                items |= 1 << map.get(&c).with_context(|| anyhow!("Incorrect item {c}"))?;
            }
            items_in_3_lines &= items;
        }
        line_no += 1;
    }

    if line_no == 1 {
        Ok(sum)
    } else {
        Err(anyhow!("Extra {} line(s)", line_no - 1))
    }
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
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 157);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 8053);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 70);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 2425);
    }
}
