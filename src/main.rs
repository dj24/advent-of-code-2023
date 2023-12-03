use std::time::Instant;

mod day1;
mod day2;

fn main() {
    let start_time = Instant::now();
    println!("Day 1 - Part 1: {}", day1::part1().unwrap());
    println!("Day 1 - Part 2: {}", day1::part2().unwrap());
    println!("Day 2 - Part 1: {}", day2::part1().unwrap());
    println!("Day 2 - Part 2: {}", day2::part2().unwrap());

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Executed in {:?}", elapsed_time);
}
