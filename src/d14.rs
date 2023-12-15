use crate::io_utils;
use crate::map_utils as mp;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d14-example.txt").unwrap();
    let mut map = lines.collect::<mp::Map>();
    // rotate once to get top row to be left column: accurate scoring
    map = mp::rotate_l(map);
    tilt_map_inplace(&mut map);
    println!("START:\n{}", map.join("\n"));
    println!("Weight: {}", score_map(mp::rotate_r(map.clone())));

    for i in 0..2 {
        run_cycle_inplace(&mut map);
        println!(
            "After Cycle {}: \n{}",
            i + 2,
            mp::rotate_r(map.clone()).join("\n")
        );
        println!("Weight: {}", score_map(mp::rotate_r(map.clone())));
    }
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
    sections.collect::<String>()[..row.len()].to_string()
}

fn tilt_map_inplace(map: &mut mp::Map) {
    for r in map.iter_mut() {
        *r = tilt_row(r.to_string());
    }
}

fn run_cycle_inplace(map: &mut mp::Map) {
    let mut map_t = mp::rotate_l(map.clone());
    tilt_map_inplace(&mut map_t);
    map_t = mp::rotate_l(map_t);
    tilt_map_inplace(&mut map_t);
    map_t = mp::rotate_l(map_t);
    tilt_map_inplace(&mut map_t);
    map_t = mp::rotate_l(map_t);
    tilt_map_inplace(&mut map_t);
    *map = map_t;
}

fn score_map(map: mp::Map) -> u32 {
    let counts = map
        .iter()
        .map(|r| r.chars().filter(|c| *c == 'O').count() as u32);
    let vals = counts
        .rev()
        .enumerate()
        .map(|(i, count)| count * (i + 1) as u32);
    vals.sum()
}

fn hash_row(row: String) -> u64 {
    let mut hash = 0;
    for (i, c) in row.chars().enumerate() {
        if c == 'O' {
            hash |= 1 << i % 64;
        }
    }
    hash
}

fn hash_map(map: mp::Map) -> u64 {
    map.iter()
        .map(|r| hash_row(r.to_string()))
        .fold(0, |a, b| a ^ b)
}
