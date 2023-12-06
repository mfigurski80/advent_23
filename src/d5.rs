use crate::io_utils;
use crate::range_utils as rng;
use std::ops::Range;

pub fn run() {
    let mut sects = io_utils::read_file_sections("inputs/d5.txt").unwrap();
    let mut seeds = parse_seeds(sects.next().unwrap());
    println!("FOUND SEEDS: {:?}", seeds);
    for offset_map in sects.map(parse_offset_map) {
        println!("FOUND OFFSET MAP: {:?}", offset_map);
        seeds = apply_offset_map(&offset_map, seeds);
        println!("NEW SEEDS: {:?}", seeds);
    }
    let min = seeds.iter().map(|s| s.start).min().unwrap();
    println!("MIN: {}", min);
}

type Seeds = Vec<Range<usize>>;

fn parse_seeds(line: String) -> Seeds {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .array_chunks::<2>()
        .map(|a| a[0]..a[0] + a[1])
        .collect()
}

#[derive(Debug)]
struct RangeOffset {
    range: Range<usize>,
    offset: isize,
}

type OffsetMap = Vec<RangeOffset>;

fn parse_offset_map(section: String) -> OffsetMap {
    section
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
        .collect()
}

/// Return modified seeds
fn apply_offset_map(map: &OffsetMap, seeds: Seeds) -> Seeds {
    let mut seeds = seeds;
    let mut new_seeds: Seeds = vec![];
    for offset in map.iter() {
        let mut leftovers: Seeds = vec![];
        for seed in seeds.iter() {
            // left & right is leftover, center is mapped
            let (left, center, right) = rng::partition_on_range(seed.clone(), &offset.range);
            if let Some(left) = left {
                leftovers.push(left);
            }
            if let Some(right) = right {
                leftovers.push(right);
            }
            if center.is_none() {
                continue; // no overlap, no change
            }
            let new_center = rng::transpose_uns(&center.unwrap(), offset.offset);
            new_seeds.push(new_center);
        }
        seeds = leftovers;
    }
    new_seeds.extend(seeds);
    return new_seeds;
}
