use std::fs;

mod days;

fn main() {
    let day= 7u8;
    let input_file_path = format!("data/day_{}_full.txt", day);
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Running day {day}!");
    match day {
        1 => {
            println!("Part 1: {}", days::day_1::part_1(&contents));
            println!("Part 2: {}", days::day_1::part_2(&contents));
        },
        2 => {
            println!("Part 1: {}", days::day_2::part_1(&contents));
            println!("Part 2: {}", days::day_2::part_2(&contents))
        },
        3 => {
            println!("Part 1: {}", days::day_3::part_1(&contents));
            println!("Part 2: {}", days::day_3::part_2(&contents))
        },
        4 => {
            println!("Part 1: {}", days::day_4::part_1(&contents));
            println!("Part 2: {}", days::day_4::part_2(&contents))
        },
        5 => {
            println!("Part 1: {}", days::day_5::part_1(&contents));
            println!("Part 2: {}", days::day_5::part_2(&contents));
        },
        6 => {
            println!("Part 1: {}", days::day_6::part_1(&contents));
            println!("Part 2: {}", days::day_6::part_2(&contents));
        },
        7 => {
            println!("Part 1: {}", days::day_7::part_1(&contents));
            println!("Part 2: {}", days::day_7::part_2(&contents));
        }
        _ => eprintln!("Day not implemented!")
    };
}
