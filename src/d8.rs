use crate::io_utils;
use std::collections::HashMap;

pub fn run() {
    let mut lines = io_utils::read_file_lines("inputs/d8-example.txt").unwrap();
    let path = parse_path(lines.next().unwrap());
    println!("Found Path: {:?}", path);
    let mut nodes: HashMap<String, [String; 2]> = HashMap::new();
    // let mut start_nodes: Vec<String> = Vec::new();
    for l in lines {
        let (id, left, right) = parse_node(l);
        nodes.insert(id.clone(), [left, right]);
        // if is_node_start(&id) {
        // start_nodes.push(id);
        // }
    }
    println!("Found Nodes: {:?}", nodes);
    // follow path infinitely
    let mut steps = 0;
    let mut current_node = "AAA".to_string();
    for dir in path.iter().cycle() {
        if current_node == "ZZZ" {
            println!("Reached ZZZ");
            break;
        }
        steps += 1;
        let children = nodes.get(&current_node).unwrap();
        let next_node = match dir {
            DIR::L => children[0].to_string(),
            DIR::R => children[1].to_string(),
        };
        println!("{} -> {}", current_node, next_node);
        current_node = next_node;
    }
    println!("Steps: {}", steps);
}

#[derive(Debug)]
enum DIR {
    R = 0,
    L = 1,
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
    let re = Regex::new("([A-Z]+)").unwrap();
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
