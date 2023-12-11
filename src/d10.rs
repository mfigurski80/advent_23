use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d10-example.txt").unwrap();

    let mut frags = lines.into_iter().map(|l| parse_line_fragments(l.as_str()));
    let start = frags.next().unwrap();
    println!("{:?}", start);
    let pipes = frags.fold(start, |mut acc, fragments| {
        combine_fragment_lines(acc.clone(), fragments)
    });
}

#[derive(Debug, Clone)]
struct PipeFragment {
    length: u32,
    up_connects: Vec<u32>,
    down_connects: Vec<u32>,
    excuse_index: Option<u32>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum TILE {
    EMPTY = b'.',
    VERTICAL = b'|',
    HORIZONTAL = b'-',
    BOTTOMLEFT = b'L',
    BOTTOMRIGHT = b'J',
    TOPLEFT = b'F',
    TOPRIGHT = b'7',
    START = b'S',
}

fn match_tile(c: u8) -> TILE {
    match c {
        b'.' => TILE::EMPTY,
        b'|' => TILE::VERTICAL,
        b'-' => TILE::HORIZONTAL,
        b'L' => TILE::BOTTOMLEFT,
        b'J' => TILE::BOTTOMRIGHT,
        b'F' => TILE::TOPLEFT,
        b'7' => TILE::TOPRIGHT,
        b'S' => TILE::START,
        _ => panic!("Unknown tile {}", c),
    }
}

use regex::Regex;

fn parse_line_fragments(line: &str) -> Vec<PipeFragment> {
    let re = Regex::new(r"([SLF][S\-]*[SJ7])|(\|)").unwrap();
    let fragments: Vec<PipeFragment> = re
        .find_iter(line)
        .map(|m| {
            let start = m.start() as u32;
            let end = m.end() as u32;
            let mut up_connects = Vec::new();
            let mut down_connects = Vec::new();
            let start_tile = match_tile(m.as_str().as_bytes()[0]);
            let end_tile = match_tile(m.as_str().as_bytes()[m.as_str().len() - 1]);
            if start_tile == TILE::VERTICAL {
                // edge case vertical
                return PipeFragment {
                    length: 1,
                    up_connects: vec![start],
                    down_connects: vec![start],
                    excuse_index: None,
                };
            }
            let mut excuse_index = None;
            if start_tile == TILE::BOTTOMLEFT {
                up_connects.push(start);
            } else if start_tile == TILE::TOPLEFT {
                down_connects.push(start);
            } else if start_tile == TILE::START {
                up_connects.push(start);
                down_connects.push(start);
                excuse_index = Some(start);
            }
            if end_tile == TILE::BOTTOMRIGHT {
                up_connects.push(end - 1);
            } else if end_tile == TILE::TOPRIGHT {
                down_connects.push(end - 1);
            } else if end_tile == TILE::START {
                up_connects.push(end - 1);
                down_connects.push(end - 1);
                excuse_index = Some(end - 1);
            }
            PipeFragment {
                length: end - start,
                up_connects,
                down_connects,
                excuse_index,
            }
        })
        .collect();
    fragments
}

use std::collections::HashMap;

fn combine_fragment_lines(top: Vec<PipeFragment>, bottom: Vec<PipeFragment>) -> Vec<PipeFragment> {
    // given multiple fragments, combine
    // ----[↓    ↓ ]-[↓    ↓ ]----
    // -[↓  ↑]--[↑    ↑]--[↑  ↓]-
    let top_down_connects: HashMap<u32, usize> = top
        .iter()
        .enumerate()
        .flat_map(|(i, f)| f.down_connects.iter().map(move |c| (*c, i)))
        .collect();
    let bottom_up_connects: HashMap<u32, usize> = bottom
        .iter()
        .enumerate()
        .flat_map(|(i, f)| f.up_connects.iter().map(move |c| (*c, i)))
        .collect();
    println!("DOWN: {:?}", top_down_connects);
    println!("UP:   {:?}", bottom_up_connects);
    bottom
}
