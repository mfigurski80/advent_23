use crate::io_utils;
use std::collections::HashSet;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d4.txt").unwrap();
    let scores = lines
        .map(|line| line.split(": ").nth(1).unwrap().to_string())
        .map(|line| parse_sets(line))
        .map(|card| score_card(&card));

    let total = scores.sum::<u32>();
    println!("Total Score: {:?}", total);
}

type Card = (HashSet<u32>, HashSet<u32>);

fn parse_sets(line: String) -> Card {
    let mut sets = line.split(" | ");
    let set1 = sets.next().unwrap();
    let set2 = sets.next().unwrap();
    let set1 = set1
        .split(" ")
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<HashSet<u32>>();
    let set2 = set2
        .split(" ")
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<HashSet<u32>>();
    (set1, set2)
}

fn score_card(card: &Card) -> u32 {
    let (winners, mine) = card;
    let mut score = 1;
    for n in winners.iter() {
        if mine.contains(n) {
            score *= 2;
        }
    }
    if score == 1 {
        0
    } else {
        score / 2
    }
}
