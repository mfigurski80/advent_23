use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d12.txt").unwrap();
    let mut valid_sum = 0;
    lines.map(parse_line).for_each(|(record, runs)| {
        // println!("\t{} to {:?}", record, runs);
        let broken_count = record.chars().filter(|c| *c == '#').count();
        let broken_need = runs.iter().sum::<usize>();
        // println!("Missing {}", broken_need - broken_count);
        let all_combs = get_all_flip_combinations(record, broken_need - broken_count);
        let valid_combs = all_combs.filter(|rec| record_match_runs(rec, &runs));
        let n_valid = valid_combs.count();
        println!("Valid: {}", n_valid);
        valid_sum += n_valid;
    });
    println!("Sum: {}", valid_sum);
}

fn parse_line(line: String) -> (String, Vec<usize>) {
    let mut parts = line.split(" ");
    let left = parts.next().unwrap().to_string();
    // let records_re = Regex::new(r"[?#]+").unwrap();
    let group_sizes = parts
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (left, group_sizes)
}

use regex::Regex;

fn record_match_runs(record: &String, runs: &Vec<usize>) -> bool {
    let record_runs = Regex::new("#+")
        .unwrap()
        .find_iter(&record)
        .map(|m| m.len())
        .collect::<Vec<usize>>();
    record_runs.iter().zip(runs.iter()).all(|(a, b)| a == b)
}

use itertools::Itertools;

fn get_all_flip_combinations(record: String, num_to_flip: usize) -> impl Iterator<Item = String> {
    let original_record = record.clone();
    let unknown = record
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if unknown.len() < num_to_flip {
        panic!("Not enough unknowns to flip");
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
 * Is there a way to turn String + Run assignment into a permutation problem?
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
 *   4,3 implies 7 #. We have 4 #, so we need to add 3 more
 *   Try every permutation of setting 2 ? into #, of which there are:
 *   (5 choose 3) = 10. Not bad. This example isn't too far off from the real thing
 *   ?#?#??#?#
 *   #####.#.# => doesnt match [4,3]
 *   ####.##.# => no
 *   ####..### => yes!
 *   ##.####.# => no
 *   ##.##.### => no
 *   ##.#.#### => no
 *   .####.### => yes!
 *   .###.#### => no
 *   Also this is a 'combination' problem instead -- order doesn't matter
 */

/* How do we determine how to assign runs to records? Permutation problem?
 * We have Records: [a,b,c] and Runs we need to distribute (or not) [2,3,4]...
 * Could be: [2,3,4], [23,4, ], [2,34, ], [2, ,34], [23, ,4], [ ,23,4], [ ,2,34], [234, , ], [ ,234, ], [ , ,234]
 * 10 in total
 * Can we represent each option as a diff? Each run needs from 1..3 records assigned to it
 *   Into: [1,1,1], [2,1,0], [1,2,0], [1,0,2], [2,0,1], [0,2,1], [0,1,2], [3,0,0], [0,3,0], [0,0,3]
 *   Still not very permutation-like
 * Can we represent each option as assigning record to run?
 *   Into: [a,b,c], [b,c, ], [a,c, ], [a, ,c], [b, ,a], [ ,b,a], [ ,a,c], [c, , ], [ ,c, ], [ , ,c]
 *   Ehh
 * Can we represent this with recursion to a simple base case? Ie [a,b] distribute [2,3]
 *   base(2,3): [2,3], [23, ], [ ,23]
 *   Larger problem can be grouped:
 *      [base(2,3),4] => [2,3,4] [23, ,4] [ ,23,4]
 *      [2,base(3,4)] => [2,3,4] [2,34, ] [2, ,34]
 *      [base(23,4),] => [23,4,] [234, ,] [ ,234,]
 *      [,base(23,4)] => [,23,4] [,234, ] [, ,234]
 *      [base(2,34),] => [2,34,] [234, ,] [ ,234,]
 *      [,base(2,34)] => [,2,34] [,234, ] [, ,234]
 * Can we simply have separate problem classes for each run?
 *    [0] => [1,1,1]
 *    [1] => [2,1,0], [1,2,0], [1,0,2]
 *    [2] => [2,0,1], [0,2,1], [0,1,2]
 * Should we just optimize with |Runs| for-loops?
 *   Where we can test [i,j,k] (diff, example 1)...
 *   for i in 0..|Runs|
 *     for j in i..|Runs|
 *       for k in j..|Runs|
 *         now i + j + k === |Runs|!!
 *         Preconditions for #1 are valid. Each index takes from 1..3
 * Maybe we should just use the `get_all_flip_permutes` function with everything?
 *   We have a giant string: .??..??...?## and need to fit Runs [1,1,3]
 *   We need 5 #s, have only 2: need 3 flips.
 *   Indexes of '?' are [1,2,5,6,10]. Every combination of 3 (5 choose 3 is 10):
 *     [1,2,5], [1,2,6], [1,2,10], [1,5,6], [1,5,10], [1,6,10], [2,5,6], [2,5,10], [2,6,10], [5,6,10]
 *     .??..??...?##
 *     .##..#.....## => doesnt match [1,1,3]
 *     .##...#....## => no
 *     .##.......### => no
 *     .#...##....## => no
 *     .#...#....### => yes!
 *     .#....#...### => yes!
 *     ..#..##....## => no
 *     ..#..#....### => yes!
 *     ..#...#...### => yes!
 *     .....###..### => no
 *   This is totally reasonable. Fuck all this permutation stuff, just operate on single line
 *   Might be hard to memoize though => not really being reduced into repeating subproblems well
 *
 */
