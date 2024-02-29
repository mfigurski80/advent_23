use crate::io_utils;
use crate::map_utils as mp;
use crate::map_utils::MapMethods;

pub fn run() {
    let map = io_utils::read_file_lines("inputs/d16.txt")
        .unwrap()
        .collect::<mp::Map>();
    map.print();
    let best = find_best_beam_start(&map);
    println!("best: {:?}", best);
}

type Point = (usize, usize);
type Direction = (i8, i8);

fn is_horizontal(direction: Direction) -> bool {
    direction.0 == 0 && direction.1 != 0
}

fn find_best_beam_start(map: &mp::Map) -> (Point, Direction) {
    let entries_iter = iter_beam_entry_points(map[0].len(), map.len());
    let hits = entries_iter
        .map(|(start, direction)| {
            print!("beaming from {:?}", start);
            find_beam_hits(start, direction, map)
        })
        .map(|hits| {
            let count = count_beam_hits(&hits);
            println!(": {}", count);
            count
        })
        .collect::<Vec<_>>();
    let best = hits
        .iter()
        .enumerate()
        .max_by_key(|(_, &hits)| hits)
        .unwrap();
    // let best_start = entries_iter.nth(best.0).unwrap();
    println!("best hits: {:?}", best.1);
    // best_start
    ((0, 0), (0, 0))
}

/// Find all beam entry points for a map
fn iter_beam_entry_points(width: usize, height: usize) -> impl Iterator<Item = (Point, Direction)> {
    let height_iter = (0..height).flat_map(move |i| [((i, 0), (1, 0)), ((i, width - 1), (-1, 0))]);
    let width_iter = (0..width).flat_map(move |j| [((0, j), (0, 1)), ((height - 1, j), (0, -1))]);
    height_iter.chain(width_iter)
}

/// Find all beam hits starting from `start` in `direction` on `map`.
fn find_beam_hits(start: Point, direction: Direction, map: &mp::Map) -> mp::Map {
    let mut v_hits = map
        .clone()
        .into_iter()
        .map(|_| String::from_utf8(vec![b'.'; map.len()]).unwrap())
        .collect::<mp::Map>();
    let mut h_hits = v_hits.clone();
    // note we maintain separate v/h hit maps for caching
    // are north/south hits equivalent? yes, going back is guaranteed to hit the same tiles
    send_beam(start, direction, map, &mut v_hits, &mut h_hits);
    append_map(&mut v_hits, h_hits);
    v_hits
}

/// Recursive body of 'find_beam_hits'
fn send_beam(
    start: Point,
    direction: Direction,
    map: &mp::Map,
    v_hits: &mut mp::Map,
    h_hits: &mut mp::Map,
) {
    let mut cur_point = start;
    let mut cur_dir = direction;
    loop {
        // check hits
        match is_horizontal(cur_dir) {
            true => h_hits.set_point(cur_point, '#'),
            false => v_hits.set_point(cur_point, '#'),
        }
        let next_point = (
            (cur_point.0 as i8 + cur_dir.1) as usize,
            (cur_point.1 as i8 + cur_dir.0) as usize,
        );
        let tile = (*map).get_point(next_point);
        if tile.is_none() {
            return;
        }
        let tile = tile.unwrap();
        // println!("tile: {:?} {:?}", tile, next_point);
        let cache_hit = match is_horizontal(cur_dir) {
            true => h_hits.get_point(next_point).unwrap() == '#',
            false => v_hits.get_point(next_point).unwrap() == '#',
        };
        if cache_hit {
            // println!("cache hit");
            break;
        }
        cur_point = next_point;
        match tile {
            '.' => {}
            '/' => {
                cur_dir = (-cur_dir.1, -cur_dir.0);
            }
            '\\' => {
                cur_dir = (cur_dir.1, cur_dir.0);
            }
            '-' => {
                if cur_dir.1 == 0 {
                    continue;
                }
                send_beam(cur_point, (-1, 0), map, v_hits, h_hits);
                cur_dir = (1, 0);
            }
            '|' => {
                if cur_dir.0 == 0 {
                    continue;
                }
                send_beam(cur_point, (0, -1), map, v_hits, h_hits);
                cur_dir = (0, 1);
            }
            other => panic!("Unknown tile: {}", other),
        };
    }
}

fn count_beam_hits(map: &mp::Map) -> usize {
    map.iter()
        .map(|s| s.chars().filter(|c| *c == '#').count())
        .sum::<usize>()
}

fn append_map(map: &mut mp::Map, new_map: mp::Map) {
    for (i, line) in new_map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '#' {
                continue;
            }
            map.set_point((i, j), '#');
        }
    }
}
