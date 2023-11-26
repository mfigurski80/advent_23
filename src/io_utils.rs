use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read file lines
/// # Examples
/// ```
/// let lines = read_file_lines("test.txt").unwrap();
/// let v = lines.map(|l| l.unwrap()).collect::<Vec<String>>();
/// assert_eq!(v, vec!["Hello", "World"]);
/// ```
pub fn read_file_lines(path: &str) -> Result<std::io::Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    return Ok(reader.lines());
}
