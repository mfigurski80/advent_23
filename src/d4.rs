use crate::io_utils;
use std::collections::HashSet;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d4.txt").unwrap();
    let scores = lines
        .map(|line| line.split(": ").nth(1).unwrap().to_string())
        .map(|line| parse_sets(line))
        .map(|card| score_card(&card))
        .collect::<Vec<u32>>();

    // init vec with all ones
    let mut card_counts = vec![1 as u32; scores.len()];
    for (i, score) in scores.iter().enumerate() {
        let mult = card_counts[i];
        let range = (i + 1)..(i + 1 + *score as usize);
        println!("+{} of each card in: {:?}", mult, range);
        for j in range {
            card_counts[j] += mult;
        }
    }
    println!("Card scores: {:?}", scores);
    println!("Card counts: {:?}", card_counts);
    println!("Total card count: {}", card_counts.iter().sum::<u32>());
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
    let matching = winners.intersection(mine).count() as u32;
    // println!("[{}] Winners: {:?}, Mine: {:?}", matching, winners, mine);
    matching
}
