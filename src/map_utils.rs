pub type Map = Vec<String>;

pub fn transpose(map: Map) -> Map {
    let mut transposed = Vec::new();
    for i in 0..map[0].len() {
        let mut row = String::new();
        for j in 0..map.len() {
            row.push(map[j].as_bytes()[i] as char);
        }
        transposed.push(row);
    }
    transposed
}
