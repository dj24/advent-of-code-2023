use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
struct Game {
    id: u32,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            max_blue: 0,
            max_red: 0,
            max_green: 0,
        }
    }
}

fn get_game_from_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(':').collect();
    let id_string = parts[0].split(' ').collect::<Vec<_>>()[1];
    let id = id_string.parse::<u32>().unwrap();
    let mut game = Game::new(id);
    let turns_string = parts[1];
    for substr_result in turns_string.split(';') {
        let colours: Vec<&str> = substr_result.split(',').collect();
        for colour_result in colours {
            let colour_amount = colour_result.split(' ').collect::<Vec<_>>()[1];
            let colour_amount = colour_amount.parse::<u32>().unwrap();
            if colour_result.contains("blue") {
                if colour_amount > game.max_blue {
                    game.max_blue = colour_amount;
                }
            }
            else if colour_result.contains("red") {
                if colour_amount > game.max_red {
                    game.max_red = colour_amount;
                }
            }
            else if colour_result.contains("green") {
                if colour_amount > game.max_green {
                    game.max_green = colour_amount;
                }
            }
        }
    }
    game
}

pub fn part1() -> io::Result<String> {
    let file = File::open("data/day2.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let game = get_game_from_line(&line);
        if game.max_red > 12 || game.max_blue > 14 || game.max_green > 13 {
            continue;
        }
        sum += game.id;
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
        sum += game.max_blue * game.max_red * game.max_green;
    }
    Ok(sum.to_string().parse().unwrap())
}