use crate::io_utils;

pub fn run() {
    let line = io_utils::read_file_lines("inputs/d15.txt")
        .unwrap()
        .next()
        .unwrap();
    let hash_sum = line
        .split(",")
        .map(holiday_hash)
        .fold(0, |acc, x| acc + x as usize);
    println!("hash_sum: {}", hash_sum);
}

fn holiday_hash(value: &str) -> u8 {
    value
        .as_bytes()
        .iter()
        .fold(0 as u8, |acc, x| acc.wrapping_add(*x).wrapping_mul(17))
}
