use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read file lines
pub fn read_file_lines(path: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty());
    Ok(lines)
}
