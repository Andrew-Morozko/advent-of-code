#![allow(dead_code)]

use anyhow::{Context, Result};
use aoc::open;
use itertools::Itertools;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

// TODO: Run benchmarks on these 3 approaches
fn parse_ring_unsafe<const WINDOW: usize>(input: &str) -> Result<usize> {
    #![allow(clippy::undocumented_unsafe_blocks)]

    let mut prev_chars = vec!['\0'; WINDOW];
    let (mut bs, mut be) = (0, 0);
    input
        .chars()
        .find_position(move |c| {
            let new_char = *c;
            let mut i = be;
            // bs - exclusive, be - inclusive
            while i != bs {
                if new_char == unsafe { *prev_chars.get_unchecked(i) } {
                    (bs, be) = (i, (be + 1) % WINDOW);
                    unsafe { *prev_chars.get_unchecked_mut(be) = new_char };
                    return false;
                }
                i = (i + WINDOW - 1) % WINDOW;
            }
            be = (be + 1) % WINDOW;
            unsafe { *prev_chars.get_unchecked_mut(be) = new_char };
            be == bs
        })
        .map(|(pos, _)| pos + 1)
        .context("Not enough items in collection")
}

fn parse_ring<const WINDOW: usize>(input: &str) -> Result<usize> {
    #![allow(clippy::indexing_slicing)]

    let mut prev_chars = vec!['\0'; WINDOW];
    let (mut bs, mut be) = (0, 0);
    input
        .chars()
        .find_position(move |c| {
            let new_char = *c;
            let mut i = be;
            // bs - exclusive, be - inclusive
            while i != bs {
                if new_char == prev_chars[i] {
                    (bs, be) = (i, (be + 1) % WINDOW);
                    prev_chars[be] = new_char;
                    return false;
                }
                i = (i + WINDOW - 1) % WINDOW;
            }
            be = (be + 1) % WINDOW;
            prev_chars[be] = new_char;
            be == bs
        })
        .map(|(pos, _)| pos + 1)
        .context("Not enough items in collection")
}

fn parse<const WINDOW: usize>(input: &str) -> Result<usize> {
    #![allow(clippy::indexing_slicing)]

    let mut prev_chars = Vec::<char>::with_capacity(WINDOW);
    input
        .chars()
        .find_position(move |c| {
            let new_char = *c;
            for i in (0..prev_chars.len()).rev() {
                if new_char == prev_chars[i] {
                    let copy_start = i + 1;
                    for j in copy_start..prev_chars.len() {
                        prev_chars[j - copy_start] = prev_chars[j];
                    }
                    prev_chars.truncate(prev_chars.len() - i);
                    // Wow, that's the first time I had to fight the borrow checker:
                    //     prev_chars[prev_chars.len() - 1] = new_char
                    // fails to compile since prev_chars is borrowed muatbly for assignment and
                    // immutably by .len()
                    let idx = prev_chars.len() - 1;
                    prev_chars[idx] = new_char;
                    return false;
                }
            }
            prev_chars.push(new_char);
            prev_chars.len() == WINDOW
        })
        .map(|(pos, _)| pos + 1)
        .context("Not enough items in collection")
}

fn part1(f: File) -> Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;
    parse::<4>(&input)
}

fn part2(f: File) -> Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;
    parse::<14>(&input)
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
    fn test_parsers() {
        let tests = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];
        for (input, r1, r2) in tests {
            assert_eq!(parse::<4>(input).unwrap(), r1);
            assert_eq!(parse_ring::<4>(input).unwrap(), r1);
            assert_eq!(parse_ring_unsafe::<4>(input).unwrap(), r1);

            assert_eq!(parse::<14>(input).unwrap(), r2);
            assert_eq!(parse_ring::<14>(input).unwrap(), r2);
            assert_eq!(parse_ring_unsafe::<14>(input).unwrap(), r2);
        }
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 1343);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 2193);
    }
}
