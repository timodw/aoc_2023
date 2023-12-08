use std::collections::HashMap;
use num::Integer;

pub fn part_1(contents: &str) -> u64 {
    let mut content_it = contents.split("\n\n");
    let instructions: Vec<bool> = content_it.next().unwrap()
    .as_bytes()
    .iter()
    .map(|c| {
        *c == b'R'
    }).collect();

    let mapping: HashMap<&str, (&str, &str)> = content_it.next().unwrap()
    .split("\n")
    .map(|l| {
        (&l[0..3], (&l[7..10], &l[12..15]))
    }).collect();

    let mut current_loc = "AAA";
    let mut steps = 0;
    while current_loc != "ZZZ" {
        let directions = mapping.get(current_loc).unwrap();
        current_loc = if instructions[steps % instructions.len()] { directions.1 } else { directions.0 };
        steps += 1;
    } 
    steps as u64
}

pub fn part_2(contents: &str) -> u64 {
    let mut content_it = contents.split("\n\n");
    let instructions: Vec<bool> = content_it.next().unwrap()
    .as_bytes()
    .iter()
    .map(|c| {
        *c == b'R'
    }).collect();

    let mapping: HashMap<&str, (&str, &str)> = content_it.next().unwrap()
    .split("\n")
    .map(|l| {
        (&l[0..3], (&l[7..10], &l[12..15]))
    }).collect();

    mapping.keys()
    .filter(|place| place.ends_with('A'))
    .map(|loc| {
        let mut current_loc = loc.clone();
        let mut steps = 0;
        while !current_loc.ends_with('Z') {
            let directions = mapping.get(current_loc).unwrap();
            current_loc = if instructions[steps % instructions.len()] { directions.1 } else { directions.0 };
            steps += 1;
        } 
        steps as u64
    })
    .reduce(|a, b| a.lcm(&b))
    .unwrap()
}