use crate::io_utils;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d10.txt").unwrap();
    let map2d: Vec<Vec<TILE>> = lines
        .into_iter()
        .map(|mut line| {
            line.insert(0, '.');
            line.push('.');
            line
        })
        .map(|line| line.bytes().map(match_tile).collect())
        .collect();
    // println!("Map: {:?}", map2d);
    let start_pos = find_tile(&map2d, TILE::START).unwrap();
    let entity_set = build_entity_from(&map2d, tile_edges, start_pos);

    println!("Seen set: {:?}", entity_set);
    println!("Seen set size / 2: {:?}", entity_set.len() / 2);
}

type Point = (usize, usize);

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum TILE {
    EMPTY = 0,
    VERTICAL = 1,
    HORIZONTAL = 2,
    BOTTOMLEFT = 3,
    BOTTOMRIGHT = 4,
    TOPLEFT = 5,
    TOPRIGHT = 6,
    START = 7,
}

fn match_tile(c: u8) -> TILE {
    match c {
        b'.' => TILE::EMPTY,
        b'|' => TILE::VERTICAL,
        b'-' => TILE::HORIZONTAL,
        b'L' => TILE::BOTTOMLEFT,
        b'J' => TILE::BOTTOMRIGHT,
        b'F' => TILE::TOPLEFT,
        b'7' => TILE::TOPRIGHT,
        b'S' => TILE::START,
        _ => panic!("Unknown tile {}", c),
    }
}

fn find_tile(map2d: &Vec<Vec<TILE>>, tile: TILE) -> Option<(usize, usize)> {
    map2d.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find_map(|(x, t)| if *t == tile { Some((x, y)) } else { None })
    })
}

#[derive(Debug, PartialEq)]
enum TILEEDGE {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn tile_edges(tile: TILE) -> Vec<TILEEDGE> {
    match tile {
        TILE::VERTICAL => vec![TILEEDGE::UP, TILEEDGE::DOWN],
        TILE::HORIZONTAL => vec![TILEEDGE::LEFT, TILEEDGE::RIGHT],
        TILE::BOTTOMLEFT => vec![TILEEDGE::UP, TILEEDGE::RIGHT],
        TILE::BOTTOMRIGHT => vec![TILEEDGE::UP, TILEEDGE::LEFT],
        TILE::TOPLEFT => vec![TILEEDGE::DOWN, TILEEDGE::RIGHT],
        TILE::TOPRIGHT => vec![TILEEDGE::DOWN, TILEEDGE::LEFT],
        TILE::START => vec![
            TILEEDGE::UP,
            TILEEDGE::DOWN,
            TILEEDGE::LEFT,
            TILEEDGE::RIGHT,
        ],
        _ => vec![],
    }
}

fn corresponding_edge(edge: &TILEEDGE) -> TILEEDGE {
    match edge {
        TILEEDGE::UP => TILEEDGE::DOWN,
        TILEEDGE::DOWN => TILEEDGE::UP,
        TILEEDGE::LEFT => TILEEDGE::RIGHT,
        TILEEDGE::RIGHT => TILEEDGE::LEFT,
    }
}

/// Somewhat complex func to build an entity (collection
/// of tiles) from a map2d, given a starting point and
/// a rule to determine how entity tiles are connected.
fn build_entity_from(
    map2d: &Vec<Vec<TILE>>,
    entity_rules: fn(TILE) -> Vec<TILEEDGE>,
    start_pos: Point,
) -> HashSet<Point> {
    let mut all_edges: VecDeque<Point> = VecDeque::new();
    let mut seen_set: HashSet<Point> = HashSet::new();
    all_edges.push_back(start_pos);
    seen_set.insert(start_pos);
    println!("Start pos: {:?}", start_pos);

    while let Some(point) = all_edges.pop_front() {
        let tile = map2d[point.0][point.1];
        // println!("Visiting {:?} {:?}", point, tile);
        let edges = entity_rules(tile);
        for edge in edges {
            let next_point = match edge {
                TILEEDGE::UP => (point.0 - 1, point.1),
                TILEEDGE::DOWN => (point.0 + 1, point.1),
                TILEEDGE::LEFT => (point.0, point.1 - 1),
                TILEEDGE::RIGHT => (point.0, point.1 + 1),
            };
            if seen_set.contains(&next_point) {
                continue;
            }
            if tile == TILE::START {
                let next_tile = map2d[next_point.0][next_point.1];
                let next_edges: Vec<TILEEDGE> = entity_rules(next_tile)
                    .iter()
                    .map(corresponding_edge)
                    .collect();
                if !next_edges.contains(&edge) {
                    continue;
                }
            }
            all_edges.push_front(next_point);
            seen_set.insert(next_point);
        }
    }
    seen_set
}
