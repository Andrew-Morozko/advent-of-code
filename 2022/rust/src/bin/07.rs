#![allow(clippy::indexing_slicing)]
// indexes are internally tracked, no nodes will be deleted, so it's safe

use anyhow::{bail, Result};
use aoc::{open, NomFinish, Pres};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::char as nchar,
    combinator::{map, map_res, opt},
    multi::fold_many0,
    sequence::{delimited, preceded, terminated},
};
use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Debug)]
enum Command<'input> {
    CdRoot,
    CdUp,
    CdDown(&'input str),
    Ls(usize),
}

type NodeID = usize;

#[derive(Debug, Default)]
struct Node<'input> {
    // map dirname => node id
    children: HashMap<&'input str, NodeID>,
    // size of files in this dir
    size: usize,
    // size of files in children + this dir
    total_size: Cell<Option<usize>>,
}

#[derive(Debug)]
struct Tree<'input> {
    // allocation for tree nodes
    nodes: Vec<Node<'input>>,
    // current path
    path: Vec<NodeID>,
}

impl<'input> Tree<'input> {
    fn new() -> Self {
        Self {
            // Initialized with an empty root node
            nodes: vec![Node::default()],
            path: Vec::new(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_command(&mut self, cmd: Command<'input>) {
        match cmd {
            Command::CdRoot => {
                self.path.clear();
            }
            Command::CdUp => {
                self.path.pop();
            }
            Command::CdDown(dir_name) => {
                let cur_dir = self.get_cur_dir();
                let node_id = self.nodes[cur_dir]
                    .children
                    .get(dir_name)
                    .copied()
                    .unwrap_or_else(|| {
                        // This is a new name, create a new empty node
                        let node_id = self.nodes.len();
                        self.nodes.push(Node::default());
                        self.nodes[cur_dir].children.insert(dir_name, node_id);
                        node_id
                    });
                self.path.push(node_id);
            }
            Command::Ls(file_size) => {
                let cur_dir = self.get_cur_dir();
                self.nodes[cur_dir].size = file_size;
            }
        }
    }

    fn get_cur_dir(&self) -> NodeID {
        // If path is empty - we're at root
        self.path.last().copied().unwrap_or(0)
    }

    fn get_total_size(&self, idx: NodeID) -> usize {
        self.nodes[idx].total_size.get().unwrap_or_else(|| {
            let total_size = self.nodes[idx]
                .children
                .values()
                .copied()
                .fold(self.nodes[idx].size, |size, child_idx| {
                    size + self.get_total_size(child_idx)
                });
            self.nodes[idx].total_size.set(Some(total_size));
            total_size
        })
    }

    fn sum_for_part_1(&self, idx: NodeID) -> (usize, usize) {
        let (total_size, mut sum_total_sizes_sub_100_000) = self.nodes[idx]
            .children
            .values()
            .copied()
            .fold((self.nodes[idx].size, 0), |acc, idx| {
                let val = self.sum_for_part_1(idx);
                (acc.0 + val.0, acc.1 + val.1)
            });
        if total_size <= 100_000 {
            sum_total_sizes_sub_100_000 += total_size;
        }
        // caching
        self.nodes[idx].total_size.set(Some(total_size));
        (total_size, sum_total_sizes_sub_100_000)
    }

    fn part_1(&self) -> usize {
        self.sum_for_part_1(0).1
    }

    fn find_min_part_2(
        &self,
        parent_idx: NodeID,
        mut cur_min: usize,
        space_to_free: usize,
    ) -> usize {
        for idx in self.nodes[parent_idx].children.values().copied() {
            let cur_dir_size = self.get_total_size(idx);
            if cur_dir_size <= space_to_free {
                // No point going over the subdirs if the parent dir is smaller than needed
                continue;
            }
            if cur_dir_size < cur_min {
                cur_min = cur_dir_size;
            }
            cur_min = self.find_min_part_2(idx, cur_min, space_to_free);
        }
        cur_min
    }
}

fn number(input: &str) -> Pres<usize> {
    map_res(take_while(|c| matches!(c, '0'..='9')), str::parse)(input)
}

fn parse_ls_output(input: &str) -> Pres<usize> {
    fold_many0(
        terminated(
            alt((
                // ignore dirs in ls output
                map(preceded(tag("dir "), is_not("\n")), |_| 0),
                // sum up the file sizes
                terminated(number, is_not("\n")),
            )),
            opt(nchar('\n')),
        ),
        || 0,
        |size, cur_size| size + cur_size,
    )(input)
}

fn parse(input: &str) -> Pres<Tree> {
    let mut tree = Tree::new();
    let res = fold_many0(
        delimited(
            tag("$ "),
            alt((
                map(
                    preceded(tag("cd "), is_not("\n")),
                    |dir_name: &str| match dir_name.trim() {
                        "/" => Command::CdRoot,
                        ".." => Command::CdUp,
                        dir_name => Command::CdDown(dir_name),
                    },
                ),
                map(preceded(tag("ls\n"), parse_ls_output), Command::Ls),
            )),
            opt(nchar('\n')),
        ),
        || {},
        |_, cmd| tree.handle_command(cmd),
    )(input);
    res.map(move |(rest, _)| (rest, tree))
}

fn part1(f: File) -> anyhow::Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;

    let tree = parse(&input).finish(&input)?;
    Ok(tree.part_1())
}

fn part2(f: File) -> Result<usize> {
    let mut input = String::new();
    BufReader::new(f).read_to_string(&mut input)?;

    let tree = parse(&input).finish(&input)?;

    let total_space = 70_000_000;
    let free_space_needed = 30_000_000;
    let total_used_space = tree.get_total_size(0);
    if total_space < total_used_space {
        bail!("total_space<total_used_space");
    }
    let current_free_space = total_space - total_used_space;
    if current_free_space >= free_space_needed {
        bail!("current_free_space>=free_space_needed");
    }
    let space_to_free = free_space_needed - current_free_space;
    if total_used_space < space_to_free {
        bail!("total_used_space<space_to_free");
    }

    Ok(tree.find_min_part_2(0, total_used_space, space_to_free))
    // parse(&input, false)
}

fn main() -> anyhow::Result<()> {
    println!("Part 1: {}", part1(open!("input.txt")?)?);
    println!("Part 2: {}", part2(open!("input.txt")?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(open!("test.txt").unwrap()).unwrap(), 95437);
    }

    #[test]
    fn test_part1_regression() {
        assert_eq!(part1(open!("input.txt").unwrap()).unwrap(), 2104783);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(open!("test.txt").unwrap()).unwrap(), 24933642);
    }

    #[test]
    fn test_part2_regression() {
        assert_eq!(part2(open!("input.txt").unwrap()).unwrap(), 5883165);
    }
}
