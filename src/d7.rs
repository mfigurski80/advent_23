use crate::io_utils;

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d7-example.txt").unwrap();
    let mut hands = lines.map(parse_hand).collect::<Vec<_>>();
    for hand in &mut hands {
        println!("{:?}", hand);
    }
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    hand_type: HandType,
    value: u64,
    bet: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum HandType {
    HighCard = 0b001,
    Pair = 0b010,
    TwoPair = 0b011,
    ThreeOfAKind = 0b100,
    FullHouse = 0b101,
    FourOfAKind = 0b110,
    FiveOfAKind = 0b111,
}

fn parse_hand(line: String) -> Hand {
    let mut sp = line.split(" ");
    let cards = sp
        .next()
        .unwrap()
        .chars()
        .map(parse_card_value)
        .collect::<Vec<u8>>();
    let bet = sp.next().unwrap().parse::<usize>().unwrap();
    let hand_type = parse_hand_type(cards.clone());
    // top three bits are hand type
    let value: u64 = (hand_type as u64) << 52;

    Hand {
        cards: cards.try_into().unwrap(),
        hand_type,
        value,
        bet,
    }
}

fn parse_card_value(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn parse_hand_type(cards: Vec<u8>) -> HandType {
    // get card counts
    let mut count = cards
        .iter()
        .fold([0; 15], |mut counts, &c| {
            counts[c as usize] += 1;
            counts
        })
        .to_vec();
    count.sort_unstable();
    let top_2: [u8; 2] = count
        .iter()
        .rev()
        .take(2)
        .map(|&c| c as u8)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    match top_2 {
        [5, 0] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1] => HandType::ThreeOfAKind,
        [2, 2] => HandType::TwoPair,
        [2, 1] => HandType::Pair,
        [1, 1] => HandType::HighCard,
        _ => panic!("invalid card counts: {:?}", count),
    }
}
