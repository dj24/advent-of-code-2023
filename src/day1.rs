use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn part1() -> io::Result<String> {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut number_string = String::new();
                for char in line.chars() {
                    if char.is_digit(10) {
                        number_string.push(char);
                        break;
                    }
                }
                for char in line.chars().rev() {
                    if char.is_digit(10) {
                        number_string.push(char);
                        break;
                    }
                }
                sum += number_string.parse::<u32>().unwrap();
            },
            Err(_) => continue,
        }
    }
    Ok(sum.to_string().parse().unwrap())
}

fn get_number_name_map() -> HashMap<String, u32> {
    let mut number_name_map: HashMap<String, u32> = HashMap::new();
    number_name_map.insert("one".to_string(), 1);
    number_name_map.insert("two".to_string(), 2);
    number_name_map.insert("three".to_string(), 3);
    number_name_map.insert("four".to_string(), 4);
    number_name_map.insert("five".to_string(), 5);
    number_name_map.insert("six".to_string(), 6);
    number_name_map.insert("seven".to_string(), 7);
    number_name_map.insert("eight".to_string(), 8);
    number_name_map.insert("nine".to_string(), 9);
    number_name_map
}

fn get_first_number_char(line: &str) -> Result<char, &str> {
    let mut sub_string = String::new();
    let number_name_map = get_number_name_map();
    for char in line.chars() {
        if char.is_digit(10) {
            return Ok(char);
        }
        else{
            sub_string = sub_string + &char.to_string();
            // println!("sub_string forwards: {}", sub_string);
            for(key, value) in &number_name_map {
                if sub_string.contains(key) {
                    // println!("found key forwards: {}", key);
                    return Ok(char::from_digit(*value, 10).unwrap());
                }
            }
        }
    }
    Err("No number found")
}

fn get_first_number_char_from_end(line: &str) -> Result<char, &str> {
    let mut sub_string = String::new();
    let number_name_map = get_number_name_map();
    for char in line.chars().rev() {
        if char.is_digit(10) {
            return Ok(char);
        }
        else{
            sub_string = char.to_string() + &sub_string;
            // println!("sub_string backwards: {}", sub_string);
            for(key, value) in &number_name_map {
                if sub_string.contains(key){
                    // println!("found key backwards: {}", key);
                    return Ok(char::from_digit(*value, 10).unwrap());
                }
            }
        }
    }
    Err("No number found")
}
pub fn part2() -> io::Result<String> {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for line_result in reader.lines() {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let line = line_result.unwrap();
            let number_string = get_first_number_char(&line).unwrap().to_string() + &get_first_number_char_from_end(&line).unwrap().to_string();
            let parsed_number = number_string.parse::<u32>().unwrap();
            let mut num = counter.lock().unwrap();
            *num += parsed_number;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *counter.lock().unwrap();

    Ok(result.to_string())
}