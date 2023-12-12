use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d12-example.txt").unwrap();
    lines.map(parse_line).for_each(|(records, groups)| {
        println!("{:?} into {:?}", groups, records);
    });
}

type ProblemState = (Vec<String>, Vec<usize>);

use regex::Regex;
fn parse_line(line: String) -> ProblemState {
    let mut parts = line.split(" ");
    let left = parts.next().unwrap();
    let records_re = Regex::new(r"[?#]+").unwrap();
    let records = records_re
        .find_iter(left)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<_>>();
    let group_sizes = parts
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (records, group_sizes)
}

fn next_problem(state: ProblemState) -> Vec<ProblemState> {
    let (records, group_sizes) = state;
    if records.len() == 0 || group_sizes.len() == 0 {
        return vec![];
    }
    let group_size = group_sizes[0];
    let record = &records[0];
    let record_len = record.len();
    if group_size > record_len {
        // return problem state wihout first record
        return vec![(records[1..].to_vec(), group_sizes)];
    }
    for ch_i in 0..record_len - group_size {
        // all possible permutations of this record in this position
        let ch = record.chars().nth(ch_i).unwrap();
        let new_record = record.clone()[ch_i..ch_i + group_size].to_string();
        let leftover_record = match record_len - ch_i - group_size - 1 {
            0 => "".to_string(),
            _ => record.clone()[ch_i + group_size + 1..].to_string(),
        };
    }

    vec![]
}

fn record_match_runs(record: Vec<String>, runs: Vec<usize>) -> bool {
    return record.len() == runs.len()
        && record
            .iter()
            .zip(runs.iter())
            .all(|(r, run)| (*r).len() == *run);
}

fn get_unknown_indexes(record: &str) -> Vec<usize> {
    record
        .chars()
        .filter(|c| *c == '?')
        .map(|c| c as usize)
        .collect()
}

use itertools::Itertools;

fn get_all_flip_permutes(record: String, num_to_flip: usize) -> impl Iterator<Item = String> {
    let original_record = record.clone();
    let unknown = get_unknown_indexes(&record);
    if unknown.len() < num_to_flip {
        panic!("Not enough unknowns to flip");
    } else if unknown.len() == num_to_flip {
        return vec![record].into_iter();
    }
    let it = unknown.iter().combinations(num_to_flip).map(|perm| {
        let mut rec = original_record.clone();
        for i in perm {
            rec.replace_range(*i..*i + 1, "#");
        }
        rec
    });
    // TODO: find a way to avoid collect?
    return it.collect::<Vec<_>>().into_iter();
}

/*
 * Is there a way to turn this into a permutation problem?
 *
 * We have: ??##??## and need to fit [3,2] into it...
 * Could be:.###..##
 *          ..###.##
 * What are the variables being permutted here?
 * Can we begin by extracting the # ranges first? [2,2] fit [3,2]?
 *   We can probably compare this to runs, and realize that we can reduce the problem:
 *   Ie. have ??#?? and [2]. Not obvious how to reduce this way though
 * Can we extract # ranges with ? added? [6,4] fit [3,2]?
 *   Easily findable: [3,2]... not useful
 *
 * Example is also too easy, try where we have more # segments than runs
 * We have: ?#?#??#?# and need to fit [4,3] into it...
 * Can we just optimize by counting how many # we're missing?
 *   4,3 implies 7 #. We have 5, so we need to add 2 more
 *   Try every permutation of setting 2 ? into #, of which there are:
 *   (5 choose 2) = 10. Not bad. This example isn't too far off from the real thing
 */
