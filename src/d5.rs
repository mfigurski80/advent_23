use crate::io_utils;
use std::ops::Range;

pub fn run() {
    let mut sects = io_utils::read_file_sections("inputs/d5-example.txt").unwrap();
    let mut seeds = parse_seeds(sects.next().unwrap());
    println!("FOUND SEEDS: {:?}", seeds);
    for offset_map in sects.map(parse_offset_map) {
        // println!("FOUND OFFSET MAP: {:?}", offset_map);
        seeds = apply_offset_map(&offset_map, &seeds);
        println!("APPLIED OFFSET MAP: {:?}", seeds);
    }
    println!("Final Min: {}", seeds.iter().min().unwrap());
}

fn parse_seeds(line: String) -> Vec<usize> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug)]
struct RangeOffset {
    range: Range<usize>,
    offset: isize,
}

type OffsetMap = Vec<RangeOffset>;

fn parse_offset_map(section: String) -> OffsetMap {
    let m: OffsetMap = section
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" ").map(|s| s.parse::<usize>().unwrap());
            let (dest_start, source_start, len) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );
            let offset = RangeOffset {
                range: source_start..source_start + len,
                offset: (dest_start as isize - source_start as isize),
            };
            offset
        })
        .collect();
    m
}

fn apply_offset_map(map: &OffsetMap, seeds: &Vec<usize>) -> Vec<usize> {
    // return modified seeds
    let mut new_seeds = seeds.clone();
    for offset in map {
        for (i, seed) in seeds.iter().enumerate() {
            if offset.range.contains(&seed) {
                println!(
                    "Offsetting seed {} by {} (in {:?})",
                    seed, offset.offset, offset.range
                );
                new_seeds[i] = (*seed as isize + offset.offset) as usize;
            }
        }
    }
    new_seeds
}
