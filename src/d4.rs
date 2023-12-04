use crate::io_utils;
use std::collections::HashSet;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d4.txt").unwrap();
    let scores = lines
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|line| parse_sets(line));
    println!("{:?}", scores);
}

fn parse_sets(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    return line
        .split(" | ")
        .map(|set| parse_set(set))
        .take(2)
        .collect::<Vec<HashSet<u32>>>();
}

fn parse_set(set: &str) -> HashSet<u32> {
    return set
        .split(" ")
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
}
