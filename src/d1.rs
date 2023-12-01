use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d1.txt").unwrap();
    let values = lines.map(|artsy| {
        println!("artsy: {}", artsy);
        let digits = get_digits(artsy);
        println!("digits: {:?}", digits);
        // find first digit
        let first_digit = digits.clone().into_iter().find(|c| c.is_digit(10)).unwrap();
        let last_digit = digits.into_iter().rev().find(|c| c.is_digit(10)).unwrap();
        let number: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        println!("number: {}", number);
        number
    });
    let collected = values.collect::<Vec<_>>();
    let sum: i32 = collected.iter().sum();
    println!("sum: {}", sum);
}

const STR_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const MAX_STR_DIGIT_SIZE: usize = 5;

fn get_digits(line: String) -> Vec<char> {
    // using 5 window, see if window start matches str_digit
    let mut digits = Vec::new();
    let mut iterator = line.chars();

    let mut window = iterator
        .clone()
        .take(MAX_STR_DIGIT_SIZE)
        .collect::<String>();
    let _ = iterator.advance_by(MAX_STR_DIGIT_SIZE);
    while window.len() > 0 {
        let digit_res = STR_DIGITS
            .iter()
            .position(|&str_digit| window.starts_with(&str_digit));
        match digit_res {
            Some(digit) => {
                digits.push(format!("{}", digit).parse::<char>().unwrap());
                let len = STR_DIGITS[digit].len();
                window = window[len..].to_string();
                window.push_str(iterator.clone().take(len).collect::<String>().as_str());
                let _ = iterator.advance_by(len);
            }
            None => {
                // try first char is normal digit?
                let ch = window.chars().next().unwrap();
                match ch.is_digit(10) {
                    true => digits.push(ch),
                    false => {}
                }
                window = window[1..].to_string();
                let next_char = iterator.next();
                if !next_char.is_none() {
                    window.push(next_char.unwrap());
                }
            }
        }
    }
    digits
}

fn format_str_digits(line: String) -> String {
    // replace all string digits with their numeric counterparts
    let mut line = line;
    for (i, str_digit) in STR_DIGITS.iter().enumerate() {
        line = line.replace(str_digit, &i.to_string());
    }
    line
}
