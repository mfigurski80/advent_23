use crate::io_utils;
use crate::map_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d14.txt").unwrap();
    let map = lines.collect::<map_utils::Map>();
    map.iter().for_each(|r| println!("{}", r));
    let map_t = map_utils::rotate(map);
    let tilted_t = map_t
        .iter()
        .map(|r| tilt_row(r.to_string()))
        .collect::<map_utils::Map>();
    let tilted = map_utils::rotate(tilted_t);
    tilted.iter().for_each(|r| println!("{}", r));
    println!("score: {}", score_map(tilted));
}

fn tilt_row(row: String) -> String {
    let sections = row.split('#').map(|s| {
        let roll_count = s.chars().filter(|c| *c == 'O').count();
        let mut section = String::new();
        for _ in 0..roll_count {
            section.push('O');
        }
        for _ in roll_count..s.len() {
            section.push('.');
        }
        section + "#"
    });
    // remove last '#'
    sections.collect::<String>()[..row.len()].to_string()
}

fn score_map(map: map_utils::Map) -> u32 {
    let counts = map
        .iter()
        .map(|r| r.chars().filter(|c| *c == 'O').count() as u32);
    let vals = counts
        .rev()
        .enumerate()
        .map(|(i, count)| count * (i + 1) as u32);
    // println!("{:?}", vals.collect::<Vec<u32>>());
    vals.sum()
}
