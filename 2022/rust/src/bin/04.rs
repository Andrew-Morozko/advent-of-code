use anyhow::{anyhow, Context, Result};
use aoc::open;
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

struct Range {
    l: u64,
    r: u64,
}

fn parse_line(s: &str) -> Result<(Range, Range)> {
    // RE is correct, don't warm about unwraps
    #![allow(clippy::unwrap_used)]

    static RE: Lazy<regex::Regex> =
        Lazy::new(|| regex::Regex::new(r#"(\d+)-(\d+),(\d+)-(\d+)"#).unwrap());
    let cap = RE
        .captures(s)
        .with_context(|| anyhow!("Can't parse line {s}"))?;
    let mut cap = cap.iter().skip(1).map(|m| {
        m.unwrap()
            .as_str()
            .parse::<u64>()
            .context("Failed to parse int")
    });
    Ok((
        Range {
            l: cap.next().unwrap()?,
            r: cap.next().unwrap()?,
        },
        Range {
            l: cap.next().unwrap()?,
            r: cap.next().unwrap()?,
        },
    ))
}

fn parse(f: File, should_count: impl Fn(&Range, &Range) -> bool) -> Result<u64> {
    let mut count = 0;
    for line in BufReader::new(f).lines() {
        let line = line.context("Failed to read line")?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (a, b) = parse_line(line)?;
        if should_count(&a, &b) {
            count += 1;
        }
    }
    Ok(count)
}

fn part1(f: File) -> Result<u64> {
    parse(f, |a, b| {
        // a|b fully contains b|a
        a.l >= b.l && a.r <= b.r || b.l >= a.l && b.r <= a.r
    })
}

fn part2(f: File) -> Result<u64> {
    parse(f, |a, b| {
        // overlaps at all
        !(a.r < b.l || a.l > b.r)
    })
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
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 2);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 602);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 4);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 891);
    }
}
