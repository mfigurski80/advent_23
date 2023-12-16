use crate::io_utils;
use std::collections::VecDeque;

pub fn run() {
    let line = io_utils::read_file_lines("inputs/d15.txt")
        .unwrap()
        .next()
        .unwrap();
    let instructions = line.split(",").map(parse_instruction).collect_vec();
    let init: [Vec<Instruction>; 256] = std::array::from_fn(|_| Vec::new());
    let box_instructions = instructions.iter().fold(init, |mut acc, x| {
        acc[holiday_hash(x.0) as usize].push(x.clone());
        acc
    });
    let box_focus = box_instructions
        .iter()
        .enumerate()
        .map(|(box_i, instructions)| {
            if instructions.is_empty() {
                return 0;
            }
            // println!("{:?}", instructions);
            let lenses = build_lens_box(instructions);
            println!("{:?}", lenses);
            lenses
                .iter()
                .enumerate()
                .map(|(lens_i, lens)| (box_i + 1) * (lens_i + 1) * (*lens as usize))
                .sum::<usize>()
        });
    println!("SUM: {}", box_focus.sum::<usize>());
}

use itertools::Itertools;

type Instruction<'a> = (&'a str, char, u8);

fn parse_instruction(line: &str) -> Instruction {
    // last char
    let last = line.chars().last().unwrap();
    if last == '-' {
        return (line.split_at(line.len() - 1).0, '-', 0);
    }
    let digit = last.to_digit(10).unwrap() as u8;
    let label = line.split_at(line.len() - 2).0;
    return (label, '=', digit);
}

fn holiday_hash(value: &str) -> u8 {
    value
        .as_bytes()
        .iter()
        .fold(0 as u8, |acc, x| acc.wrapping_add(*x).wrapping_mul(17))
}

fn build_lens_box(instructions: &[Instruction]) -> Vec<u8> {
    let mut lenses: VecDeque<(&str, u8)> = VecDeque::new();
    for inst in instructions {
        let same_label_index = lenses.iter().position(|(label, _)| label == &inst.0);
        if inst.1 == '-' && same_label_index.is_some() {
            lenses.remove(same_label_index.unwrap());
        } else if inst.1 == '=' && same_label_index.is_some() {
            lenses[same_label_index.unwrap()].1 = inst.2;
        } else if inst.1 == '=' && same_label_index.is_none() {
            lenses.push_back((inst.0, inst.2));
        }
    }
    lenses.iter().map(|(_, lens)| *lens).collect_vec()
}
