use crate::io_utils;

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
            row_matches += fold_index + 1;
            continue;
        }
        let transposed = transpose_map(
            map.split('\n')
                .map(|r| r.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        )
        .iter()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<String>>();
        let col_hashes: Vec<u64> = transposed
            .iter()
            .map(|col| hash_row(col.to_string()))
            .collect();
        let fold_col_index = find_fold_index(col_hashes);
        if let Some(fold_index) = fold_col_index {
            println!("fold_index: COL {}", fold_index);
            col_matches += fold_index + 1;
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

fn find_fold_index(row_hashes: Vec<u64>) -> Option<usize> {
    for i in 1..(row_hashes.len()) {
        let (left, right) = row_hashes.split_at(i);
        let valid = left.iter().rev().zip(right.iter()).all(|(l, r)| l == r);
        if valid {
            return Some(i - 1);
        }
    }
    None
}

fn transpose_map<T: Copy>(map: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = Vec::new();
    for i in 0..map[0].len() {
        let mut row = Vec::new();
        for j in 0..map.len() {
            row.push(map[j][i]);
        }
        transposed.push(row);
    }
    transposed
}
