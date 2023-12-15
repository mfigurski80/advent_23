use crate::io_utils;
use crate::map_utils;

pub fn run() {
    let map_iter = io_utils::read_file_sections("inputs/d13.txt").unwrap();
    let mut col_matches = 0;
    let mut row_matches = 0;
    for map in map_iter {
        let row_hashes: Vec<u64> = map
            .split('\n')
            .map(|row| hash_row(row.to_string()))
            .collect();
        let fold_row_index = find_fold_index(row_hashes);
        if let Some(fold_index) = fold_row_index {
            println!("fold_index: ROW {}", fold_index);
            row_matches += fold_index;
            continue;
        }
        let transposed = map_utils::rotate_r(
            map.split('\n')
                .map(|r| r.to_string())
                .collect::<Vec<String>>(),
        );
        let col_hashes: Vec<u64> = transposed
            .iter()
            .map(|col| hash_row(col.to_string()))
            .collect();
        let fold_col_index = find_fold_index(col_hashes);
        if let Some(fold_index) = fold_col_index {
            println!("fold_index: COL {}", fold_index);
            col_matches += fold_index;
        }
    }
    println!("col_matches: {}", col_matches);
    println!("row_matches: {}", row_matches);
    println!("sum: {}", col_matches + 100 * row_matches);
}

fn hash_row(row: String) -> u64 {
    let mut hash = 0;
    for (i, c) in row.chars().enumerate() {
        if c == '#' {
            hash |= 1 << i % 64;
        }
    }
    hash
}

fn hash_row_dif(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

fn find_fold_index(row_hashes: Vec<u64>) -> Option<usize> {
    for i in 1..(row_hashes.len()) {
        let (left, right) = row_hashes.split_at(i);
        let mut used_smudge = false;
        let valid = left.iter().rev().zip(right.iter()).all(|(l, r)| {
            let dif = hash_row_dif(*l, *r);
            if dif == 1 && !used_smudge {
                used_smudge = true;
                return true;
            }
            return dif == 0;
        });
        if valid && used_smudge {
            return Some(i);
        }
    }
    None
}
