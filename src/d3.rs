use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d3-example.txt").unwrap();
    let map_2d = lines.collect::<Vec<String>>();
    let ent_map = map_2d
        .iter()
        .map(|line| parse_entities(line))
        .map(|ents| filter_spaces(ents))
        .collect::<Vec<Vec<(usize, Entity)>>>();
    // for each 'Symbol' entity
    let engine_parts = ent_map.iter().enumerate().flat_map(|(j, row)| {
        row.iter()
            .map(move |ent| (j, ent.0, ent.1))
            .filter(|ent| match ent.2 {
                Entity::Symbol(_) => true,
                _ => false,
            })
            .map(|ent| {
                (
                    ent.0,
                    ent.1,
                    match ent.2 {
                        Entity::Symbol(c) => c,
                        _ => panic!("filter_spaces called on non-symbol entity"),
                    },
                )
            })
    });
    let mut tot_sum = 0;
    for (row, col, part) in engine_parts {
        let adj_nums = filter_adjacent_numbers(ent_map[row].clone(), col);
        let adj_nums = (row != 0)
            .then_some(adj_nums.chain(filter_adjacent_numbers(ent_map[row - 1].clone(), col)))
            .into_iter()
            .flatten();
        let adj_nums = (row != ent_map.len() - 1)
            .then_some(adj_nums.chain(filter_adjacent_numbers(ent_map[row + 1].clone(), col)))
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, i32)>>();
        let sum = adj_nums.iter().map(|(_, val)| val).sum::<i32>();
        tot_sum += sum;

        println!("Part {} [Sum {}]: {:?}", part, sum, adj_nums);
    }
    println!("Total sum: {}", tot_sum);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entity {
    Space,
    Symbol(char),
    Number(i32),
}

fn parse_entities(row: &String) -> Vec<(usize, Entity)> {
    let ents = row.chars().enumerate().collect::<Vec<(usize, char)>>();
    // combine all consecutive 'number' entities into one
    let mut clean: Vec<(usize, Entity)> = Vec::new();
    for ent in ents.iter() {
        if !ent.1.is_numeric() {
            // not a number, just push it
            if ent.1 == '.' {
                clean.push((ent.0, Entity::Space));
            } else {
                clean.push((ent.0, Entity::Symbol(ent.1.to_owned())));
            }
            continue;
        }
        let last = clean.pop();
        if last.is_none() {
            // last is none, push current
            clean.push((ent.0, Entity::Number(ent.1.to_digit(10).unwrap() as i32)));
            continue;
        }
        let last = last.unwrap();
        match last.1 {
            Entity::Number(n) => {
                // last is a number, combine with current
                clean.push((
                    last.0,
                    Entity::Number(n * 10 + ent.1.to_digit(10).unwrap() as i32),
                ));
            }
            _ => {
                // last is not a number, push both
                clean.push(last);
                clean.push((ent.0, Entity::Number(ent.1.to_digit(10).unwrap() as i32)));
            }
        }
    }
    clean
}

fn filter_spaces(ents: Vec<(usize, Entity)>) -> Vec<(usize, Entity)> {
    ents.into_iter()
        .filter(|ent| match ent.1 {
            Entity::Space => false,
            _ => true,
        })
        .collect::<Vec<(usize, Entity)>>()
}

fn filter_adjacent_numbers(
    ents: Vec<(usize, Entity)>,
    idx: usize,
) -> impl Iterator<Item = (usize, i32)> {
    ents.into_iter()
        .filter(move |ent| match ent.1 {
            Entity::Number(val) => {
                let len = number_len(val);
                let range = expand_range(ent.0..ent.0 + len, 1);
                range.contains(&idx)
            }
            _ => false,
        })
        .map(|ent| match ent.1 {
            Entity::Number(val) => (ent.0, val),
            _ => panic!("filter_adjacent_numbers called on non-number entity"),
        })
}

fn expand_range(range: std::ops::Range<usize>, len: usize) -> std::ops::Range<usize> {
    let start = range.start.saturating_sub(len);
    let end = range.end.saturating_add(len);
    start..end
}

fn number_len(num: i32) -> usize {
    // log 10
    (num as f64).log10().floor() as usize + 1
}
