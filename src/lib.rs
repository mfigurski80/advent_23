mod io_utils;
use io_utils::read_file_lines;

pub fn main() {
    let lines = read_file_lines("test.txt").unwrap();
    let v = lines.map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("{:?}", v);
}
