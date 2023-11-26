mod io_utils;

fn main() {
    let lines = io_utils::read_file_lines("test.txt").unwrap();
    let v = lines.map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("{:?}", v);
}
