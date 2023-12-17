pub type Map = Vec<String>;

pub fn rotate_r(map: Map) -> Map {
    let mut rotated = Vec::new();
    let row_len = map.len();
    for i in 0..map[0].len() {
        let mut row = String::with_capacity(row_len);
        for j in 0..row_len {
            row.push(map[row_len - j - 1].as_bytes()[i] as char);
        }
        rotated.push(row);
    }
    rotated
}

pub fn rotate_l(map: Map) -> Map {
    rotate_r(map)
        .iter()
        .rev()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
}

pub fn transpose(map: Map) -> Map {
    let mut transposed = Vec::new();
    let row_len = map.len();
    for i in 0..map[0].len() {
        let mut row = String::with_capacity(row_len);
        for j in 0..row_len {
            row.push(map[j].as_bytes()[i] as char);
        }
        transposed.push(row);
    }
    transposed
}

pub type Point = (usize, usize);

pub trait MapMethods {
    fn print(&self);
    fn get_point(&self, point: Point) -> Option<char>;
    fn set_point(&mut self, point: Point, c: char);
}

impl MapMethods for Map {
    fn print(&self) {
        self.iter()
            .enumerate()
            .for_each(|(i, line)| println!("{:>2}: {}", i, line));
    }
    fn get_point(&self, point: Point) -> Option<char> {
        self.get(point.0)
            .unwrap_or(&"".to_owned())
            .chars()
            .nth(point.1)
    }

    fn set_point(&mut self, point: Point, c: char) {
        let mut row = self.get(point.0).unwrap().to_string();
        row.replace_range(point.1..=point.1, &c.to_string());
        self[point.0] = row;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_r() {
        let map = vec!["abc".to_string(), "def".to_string()];
        let rotated = rotate_r(map);
        assert_eq!(
            rotated,
            vec!["da".to_string(), "eb".to_string(), "fc".to_string()]
        );
        let rotated = rotate_r(rotated);
        assert_eq!(rotated, vec!["fed".to_string(), "cba".to_string()]);
    }

    #[test]
    fn test_rotate_l() {
        let map = rotate_r(vec!["abc".to_string(), "def".to_string()]);
        let rotated = rotate_l(map);
        assert_eq!(rotated, vec!["abc".to_string(), "def".to_string()]);
    }

    #[test]
    fn test_transpose() {
        let map = vec!["abc".to_string(), "def".to_string()];
        let transposed = transpose(map);
        assert_eq!(
            transposed,
            vec!["ad".to_string(), "be".to_string(), "cf".to_string()]
        );
    }
}
