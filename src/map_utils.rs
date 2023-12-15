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
}
