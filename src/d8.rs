use crate::io_utils;
use std::collections::HashMap;

pub fn run() {
    let mut lines = io_utils::read_file_lines("inputs/d8.txt").unwrap();
    let path = parse_path(lines.next().unwrap());
    println!("Found Path: {:?}", path);
    let mut nodes: HashMap<String, [String; 2]> = HashMap::new();
    let mut start_nodes: Vec<String> = Vec::new();
    for l in lines {
        let (id, left, right) = parse_node(l);
        nodes.insert(id.clone(), [left, right]);
        if is_node_start(&id) {
            start_nodes.push(id);
        }
    }
    println!("Nodes: {:?}", nodes);
    // follow path infinitely
    let mut steps = 0;
    for dir in path.iter().cycle() {
        println!("Step {}, Nodes: {:?}", steps, start_nodes);
        steps += 1;
        let dir_i: usize = *dir as usize;
        start_nodes.iter_mut().for_each(|n| {
            let children = nodes.get(n).unwrap();
            let next_node = children[dir_i].to_string();
            *n = next_node;
        });
        if start_nodes.iter().all(|n| is_node_end(n)) {
            break;
        }
    }
    println!("Steps: {}", steps);
}

#[derive(Debug, Copy, Clone)]
enum DIR {
    L = 0,
    R = 1,
}

fn parse_path(line: String) -> Vec<DIR> {
    line.chars()
        .map(|c| match c {
            'R' => DIR::R,
            'L' => DIR::L,
            _ => panic!("unexpected path direction"),
        })
        .collect()
}

use regex::Regex;

fn parse_node(line: String) -> (String, String, String) {
    let re = Regex::new("([A-Z0-9]{3})").unwrap();
    let matches = re
        .find_iter(&line)
        .take(3)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();
    (
        matches[0].to_owned(),
        matches[1].to_owned(),
        matches[2].to_owned(),
    )
}

fn is_node_start(id: &String) -> bool {
    id.chars().last().unwrap() == 'A'
}

fn is_node_end(id: &String) -> bool {
    id.chars().last().unwrap() == 'Z'
}
