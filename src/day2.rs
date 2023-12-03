use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Turn {
    blue_count: u32,
    red_count: u32,
    green_count: u32,
}

impl Turn {
    fn new() -> Turn {
        Turn {
            blue_count: 0,
            red_count: 0,
            green_count: 0,
        }
    }

    fn add_blue(&mut self, amount: u32) {
        self.blue_count += amount;
    }

    fn add_red(&mut self, amount: u32) {
        self.red_count += amount;
    }

    fn add_green(&mut self, amount: u32) {
        self.green_count += amount;
    }
}

struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            turns: Vec::new(),
        }
    }

    fn add_turns(&mut self, turns: Vec<Turn>) {
        for turn in turns {
            self.turns.push(turn);
        }
    }
}

// Returns array of turns or empty array if above possible limits
fn get_turns_from_game_string(game_string: &str) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::new();
    for substr_result in game_string.split(';') {
        let colours: Vec<&str> = substr_result.split(',').collect();
        let mut turn = Turn::new();
        for colour_result in colours {
            let colour_amount = colour_result.split(' ').collect::<Vec<_>>()[1];
            let colour_amount = colour_amount.parse::<u32>().unwrap();
            if colour_result.contains("blue") {
                if colour_amount > 14 {
                    return Vec::new();
                }
                turn.add_blue(colour_amount);
            }
            else if colour_result.contains("red") {
                if colour_amount > 12 {
                    return Vec::new();
                }
                turn.add_red(colour_amount);
            }
            else if colour_result.contains("green") {
                if colour_amount > 13 {
                    return Vec::new();
                }
                turn.add_green(colour_amount);
            }
        }
        turns.push(turn);
    }
    turns
}


fn get_game_from_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();
    let id_string = parts[0].split(' ').collect::<Vec<_>>()[1];
    let id = id_string.parse::<u32>().unwrap();
    let mut game = Game::new(id);
    let turns = parts[1];
    game.add_turns(get_turns_from_game_string(turns));
    game
}

pub fn part1() -> io::Result<String> {
    let file = File::open("data/day2.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let game = get_game_from_line(&line);
        if game.turns.len() > 0  {
            sum += game.id;
        }
    }
    Ok(sum.to_string().parse().unwrap())
}

pub fn part2() -> io::Result<String> {
    let file = File::open("data/day2.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let game = get_game_from_line(&line);

    }
    Ok(sum.to_string().parse().unwrap())
}