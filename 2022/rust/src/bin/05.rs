use anyhow::{bail, Context, Result};
use aoc::open;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{anychar, char as nchar},
    combinator::{map, map_res},
    error::{convert_error, VerboseError},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn one_crate(input: &str) -> IResult<&str, Option<char>, VerboseError<&str>> {
    // "[a]" or "   "
    alt((
        map(delimited(nchar('['), anychar, nchar(']')), Some),
        map(tag("   "), |_| None),
    ))(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>, VerboseError<&str>> {
    //     [b]
    // [c] [d]
    separated_list1(nchar('\n'), separated_list1(nchar(' '), one_crate))(input)
}

fn finalized_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>, VerboseError<&str>> {
    //     [b]
    // [c] [d]
    //  1   2
    map_res(
        terminated(
            terminated(stacks, nchar('\n')),
            tuple((take_while(|c| matches!(c, ' ' | '0'..='9')), nchar('\n'))),
        ),
        |v| -> Result<_> {
            // need to rotate vecs
            let mut res = vec![Vec::<char>::new(); v.last().map_or(0, Vec::len)];
            for row in v.into_iter().rev() {
                for (i, c) in row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, c)| c.map(|c| (i, c)))
                {
                    res.get_mut(i)
                        .map(|v| v.push(c))
                        .context("Line wider than base")?;
                }
            }
            Result::Ok(res)
        },
    )(input)
}

fn number(input: &str) -> IResult<&str, usize, VerboseError<&str>> {
    map_res(take_while(|c| matches!(c, '0'..='9')), str::parse)(input)
}

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn one_move(input: &str) -> IResult<&str, Move, VerboseError<&str>> {
    map(
        tuple((
            preceded(tag("move "), number),
            preceded(tag(" from "), number),
            preceded(tag(" to "), number),
        )),
        |(count, from, to)| Move { count, from, to },
    )(input)
}

fn moves(input: &str) -> IResult<&str, Vec<Move>, VerboseError<&str>> {
    separated_list1(nchar('\n'), one_move)(input)
}

fn parse(input: &str, should_reverse: bool) -> Result<String> {
    let res = map(
        separated_pair(finalized_stacks, nchar('\n'), moves),
        |(mut s, moves)| -> Result<String> {
            for Move { count, from, to } in moves {
                if from == to {
                    // Noop
                    continue;
                }
                let source = s.get_mut(from - 1).context("wrong 'from'")?;
                if count > source.len() {
                    bail!("Can't take more than in stack");
                }
                let split = source.split_off(source.len() - count);
                let dest = s.get_mut(to - 1).context("wrong 'to'")?;
                if should_reverse {
                    dest.extend(split.into_iter().rev());
                } else {
                    dest.extend_from_slice(&split);
                }
            }
            Ok(s.into_iter().map(|s| *s.last().unwrap_or(&' ')).collect())
        },
    )(input)
    .finish();
    match res {
        Ok((_, res)) => res,
        Err(e) => bail!("Parse error:\n{}", convert_error(input, e)),
    }
}

fn part1(f: File) -> Result<String> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;

    parse(&input, true)
}

fn part2(f: File) -> Result<String> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;

    parse(&input, false)
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
    fn nom_tests() {
        assert_eq!(one_crate("[a]"), Ok(("", Some('a'))));
        assert_eq!(one_crate("   "), Ok(("", None)));
        assert_eq!(
            stacks("    [b]\n[c] [d]"),
            Ok(("", vec![vec![None, Some('b')], vec![Some('c'), Some('d')]]))
        );
        assert_eq!(
            finalized_stacks("    [b]\n[c] [d]\n 1   2\n"),
            Ok(("", vec![vec!['c'], vec!['d', 'b']]))
        );

        assert_eq!(
            one_move("move 1 from 2 to 1"),
            Ok((
                "",
                Move {
                    count: 1,
                    from: 2,
                    to: 1
                }
            ))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), "CMZ");
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), "FRDSQRRCD");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), "MCD");
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), "HRFTQVWNN");
    }
}
