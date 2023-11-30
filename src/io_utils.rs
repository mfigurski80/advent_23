use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;
use std::slice::Chunks;

/// Read file lines
pub fn read_file_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    return Ok(reader.lines());
}

/// Read by n file lines. Every returned strings has n lines.
pub fn read_by_n_file_lines(path: &str, n: usize) -> Result<Map<String, String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // return generator
    return Ok(reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .chunks(n)
        .map(|c| c.join("\n")));
}
