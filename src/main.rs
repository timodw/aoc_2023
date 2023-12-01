use std::env;
use std::fs;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid arguments!");
        std::process::exit(1);
    }
    let day: u8 = match args[1].parse::<u8>() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: could not parse day value!");
            std::process::exit(1);
        }
    };
    println!("Running day {day}!");
    match day {
        1 => {
            let input_file_path = "data/day_1_full.txt";
            let contents = fs::read_to_string(input_file_path).unwrap();
            println!("Part 1: {}", days::day_1::part_1(&contents));
            println!("Part 2: {}", days::day_1::part_2(&contents));
        },
        _ => eprintln!("Day not implemented!")
    };
}
