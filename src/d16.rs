use crate::io_utils;
use crate::map_utils as mp;
use crate::map_utils::MapMethods;

pub fn run() {
    let map = io_utils::read_file_lines("inputs/d16.txt")
        .unwrap()
        .collect::<mp::Map>();
    map.print();
    let hits = send_beam((0, 0), (0, 1), &map, 600);
    hits.print();
    let hits_count = hits
        .iter()
        .map(|s| s.chars().filter(|c| *c == '#').count())
        .sum::<usize>();
    println!("hits_count: {}", hits_count);
}

type Point = (usize, usize);
type Direction = (i8, i8);

fn send_beam(start: Point, direction: Direction, map: &mp::Map, n: usize) -> mp::Map {
    let mut hits = map
        .clone()
        .into_iter()
        .map(|_| String::from_utf8(vec![b'.'; map.len()]).unwrap())
        .collect::<mp::Map>();
    let mut cur_point = start;
    let mut cur_dir = direction;
    for i in 0..n {
        hits.set_point(cur_point, '#');
        let next_point = (
            (cur_point.0 as i8 + cur_dir.1) as usize,
            (cur_point.1 as i8 + cur_dir.0) as usize,
        );
        let tile = (*map).get_point(next_point);
        if tile.is_none() {
            // println!("RAN OFF END");
            break;
        }
        let tile = tile.unwrap();
        // println!("tile: {:?} {:?}", tile, next_point);
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
                // println!("splitting map");
                let split_map_hits = send_beam(cur_point, (-1, 0), map, n - i);
                append_map(&mut hits, split_map_hits);
                println!("new_map_hits");
                hits.print();
                cur_dir = (1, 0);
            }
            '|' => {
                if cur_dir.0 == 0 {
                    continue;
                }
                // println!("splitting map");
                let split_map_hits = send_beam(cur_point, (0, -1), map, n - i);
                append_map(&mut hits, split_map_hits);
                // println!("new_map_hits:");
                // hits.print();
                cur_dir = (0, 1);
            }
            _ => panic!("Unknown tile: {}", tile),
        };
    }
    hits
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
