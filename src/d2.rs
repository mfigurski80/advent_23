use crate::io_utils;

type Game = (u32, u32, u32);

const MAX_GAME: Game = (12, 13, 14);

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d2.txt").unwrap();

    let mut power_sum = 0;
    for line in lines {
        let (_, games_str) = match_id(&line);
        let games = match_games(games_str);
        // get max of all games
        let max = games_max(games);
        power_sum += game_power(max);
    }
    println!("Sum of max games power: {}", power_sum);
}

fn match_id(line: &str) -> (i32, &str) {
    let id = line
        .split(": ")
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    (id, line.split(": ").nth(1).unwrap())
}

fn match_games(games_str: &str) -> Vec<Game> {
    games_str
        .split("; ")
        .map(|game_str| {
            let mut game = (0, 0, 0);
            for result in game_str.split(", ").take(3) {
                let (score, color) = match_result(result);
                match color {
                    "red" => game.0 = score,
                    "green" => game.1 = score,
                    "blue" => game.2 = score,
                    _ => panic!("Invalid color"),
                }
            }
            game
        })
        .collect()
}

fn match_result(result: &str) -> (u32, &str) {
    let score = result.split(" ").next().unwrap().parse::<u32>().unwrap();
    let color = result.split(" ").nth(1).unwrap();
    (score, color)
}

fn games_max(games: Vec<Game>) -> Game {
    let mut max_game = (0, 0, 0);
    for game in games {
        if game.0 > max_game.0 {
            max_game.0 = game.0;
        }
        if game.1 > max_game.1 {
            max_game.1 = game.1;
        }
        if game.2 > max_game.2 {
            max_game.2 = game.2;
        }
    }
    max_game
}

fn game_power(game: Game) -> u32 {
    game.0 * game.1 * game.2
}
