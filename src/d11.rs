use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d11.txt").unwrap();
    let mut galaxy_points = parse_galaxies(lines);
    println!("Galaxies: {:?}", galaxy_points);
    apply_y_expansion(&mut galaxy_points);
    galaxy_points.sort_unstable_by_key(|(_, x)| *x);
    apply_x_expansion(&mut galaxy_points);
    galaxy_points.sort_unstable_by_key(|(y, _)| *y);
    println!("Galaxies: {:?}", galaxy_points);
    let distance = get_distance_matrix(&galaxy_points);
    println!(
        "Distance: {:?}",
        distance.iter().flatten().sum::<usize>() / 2
    );
}

type Point = (usize, usize);

enum TILE {
    SPACE = 0,
    GALAXY = 1,
}

fn match_tile(c: char) -> TILE {
    match c {
        '.' => TILE::SPACE,
        '#' => TILE::GALAXY,
        _ => panic!("Unknown tile"),
    }
}

/// Parse galaxy points from iterator. Returns sorted by y, then x
fn parse_galaxies(lines: impl Iterator<Item = String>) -> Vec<Point> {
    let mut galaxies = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match match_tile(c) {
                TILE::GALAXY => galaxies.push((y, x)),
                _ => (),
            }
        }
    }
    galaxies
}

/// Apply expansion rules to galaxies. Input must be sorted by y
fn apply_y_expansion(galaxies: &mut Vec<Point>) {
    // for every empty y, all consecutive galaxies are shifted by 1
    let mut shift = 0;
    let mut last_y: usize = 0;
    galaxies.iter_mut().for_each(|(y, _)| {
        let count_skipped = (*y).saturating_sub(last_y).saturating_sub(1);
        shift += count_skipped;
        last_y = *y;
        if shift > 0 {
            *y += shift * 1000000;
        }
    });
}

/// Apply expansion rules to galaxies. Input must be sorted by x
fn apply_x_expansion(galaxies: &mut Vec<Point>) {
    let mut shift = 0;
    let mut last_x: usize = 0;
    galaxies.iter_mut().for_each(|(_, x)| {
        let count_skipped = (*x).saturating_sub(last_x).saturating_sub(1);
        shift += count_skipped;
        last_x = *x;
        if shift > 0 {
            *x += shift * 1000000;
        }
    });
}

fn get_distance_matrix(galaxies: &Vec<Point>) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; galaxies.len()]; galaxies.len()];
    for (i, (y1, x1)) in galaxies.iter().enumerate() {
        for (j, (y2, x2)) in galaxies.iter().enumerate() {
            matrix[i][j] = (*y1 as isize - *y2 as isize).abs() as usize
                + (*x1 as isize - *x2 as isize).abs() as usize;
        }
    }
    // for i in 0..galaxies.len() {
    // matrix[i][i] = usize::MAX;
    // }
    matrix
}
