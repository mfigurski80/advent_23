use crate::io_utils;
use crate::map_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d14-example.txt").unwrap();
    let map = lines.collect::<map_utils::Map>();
    // map.iter().for_each(|r| println!("{}", r));
    let map_t = map_utils::transpose(map);
    let tilted = map_t
        .iter()
        .map(|r| {
            println!("{}", r);
            r.to_owned()
        })
        .collect::<map_utils::Map>();
}
