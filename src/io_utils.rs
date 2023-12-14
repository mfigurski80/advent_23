use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read file lines
pub fn read_file_lines(path: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty());
    Ok(lines)
}

/// Read file by section (\n\n)
pub fn read_file_sections(path: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sections = reader
        .lines()
        .batching(|lines| {
            let mut section = String::new();
            for line in lines.by_ref().map_while(Result::ok) {
                if line.is_empty() {
                    return Some(section.trim().to_string());
                } else {
                    section.push_str(&line);
                    section.push('\n');
                }
            }
            if section.is_empty() {
                None
            } else {
                Some(section.trim().to_string())
            }
        })
        .filter(|section| !section.is_empty());
    Ok(sections)
}

use regex::Regex;
use std::io::prelude::*;

/// Read file by multiline regex matches
/// NOTE: this is very bad
pub fn read_file_matching_reg(
    path: &str,
    reg: &str,
) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let reg_obj = Regex::new(reg).unwrap();
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let matches = reg_obj
        .find_iter(&contents)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();
    Ok(matches.into_iter())
}
