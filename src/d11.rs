use crate::io_utils;

const EXPANSION_FACTOR: u32 = 999999;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d11.txt").unwrap();
    let mut galaxy_points = parse_galaxies(lines);
    println!("Galaxies: {:?}", galaxy_points);
    apply_y_expansion(&mut galaxy_points);
    galaxy_points.sort_unstable_by_key(|(_, x)| x.clone());
    apply_x_expansion(&mut galaxy_points);
    galaxy_points.sort_unstable_by_key(|(y, _)| y.clone());
    println!("Galaxies: {:?}", galaxy_points);
    let distance = get_distance_matrix(&galaxy_points);
    println!(
        "Distance: {:?}",
        distance.iter().flatten().sum::<BigUint>() / 2 as u8
    );
}

use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};

type Point = (BigUint, BigUint);

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
                TILE::GALAXY => galaxies.push((y.to_biguint().unwrap(), x.to_biguint().unwrap())),
                _ => (),
            }
        }
    }
    galaxies
}

/// Apply expansion rules to galaxies. Input must be sorted by y
fn apply_y_expansion(galaxies: &mut Vec<Point>) {
    // for every empty y, all consecutive galaxies are shifted by 1
    let mut shift: BigUint = Zero::zero();
    let mut last_y: BigUint = One::one();
    galaxies.iter_mut().for_each(|(y, _)| {
        let count_skipped: BigUint = match *y > last_y {
            true => &*y - last_y.clone(),
            false => Zero::zero(),
        };
        // println!("count_skipped: {}", count_skipped);
        shift += count_skipped;
        last_y = y.clone() + 1 as u8;
        *y += shift.clone() * EXPANSION_FACTOR;
    });
}

/// Apply expansion rules to galaxies. Input must be sorted by x
fn apply_x_expansion(galaxies: &mut Vec<Point>) {
    let mut shift: BigUint = Zero::zero();
    let mut last_x: BigUint = One::one();
    galaxies.iter_mut().for_each(|(_, x)| {
        let count_skipped = match *x > last_x {
            true => &*x - last_x.clone(),
            false => Zero::zero(),
        };
        // println!("count_skipped: {}", count_skipped);
        shift += count_skipped;
        last_x = x.clone() + 1 as u8;
        *x += shift.clone() * EXPANSION_FACTOR;
    });
}

fn get_distance_matrix(galaxies: &Vec<Point>) -> Vec<Vec<BigUint>> {
    let mut matrix = vec![vec![Zero::zero(); galaxies.len()]; galaxies.len()];
    for (i, (y1, x1)) in galaxies.iter().enumerate() {
        for (j, (y2, x2)) in galaxies.iter().enumerate() {
            // matrix[i][j] = (*y1 as isize - *y2 as isize).abs() as usize
            // + (*x1 as isize - *x2 as isize).abs() as usize;
            let x = abs_diff(x1.to_owned(), x2.to_owned());
            let y = abs_diff(y1.to_owned(), y2.to_owned());
            matrix[i][j] = (y) + (x);
        }
    }
    matrix
}

fn abs_diff(a: BigUint, b: BigUint) -> BigUint {
    match a > b {
        true => a - b,
        false => b - a,
    }
}
