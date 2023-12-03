use std::time::Instant;

mod day1;
fn main() {
    let start_time = Instant::now();
    match day1::part1() {
        Ok(content) => println!("{}", content),
        Err(err) => eprintln!("{}", err),
    }
    match day1::part2() {
        Ok(content) => println!("{}", content),
        Err(err) => eprintln!("{}", err),
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Executed in: {:?}", elapsed_time);
}
