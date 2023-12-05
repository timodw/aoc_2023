use std::collections::{HashSet, HashMap};

fn count_matches(line: &str) -> u64 {
    let mut card_numbers = line.split(" | ");
    let winning_numbers: HashSet<u64> = card_numbers.next().unwrap().split_whitespace()
                                            .map(|number| number.parse::<u64>().unwrap()).collect();
    let own_numbers: HashSet<u64> = card_numbers.next().unwrap().split_whitespace()
                                        .map(|number| number.parse::<u64>().unwrap()).collect();
    own_numbers.intersection(&winning_numbers).count() as u64
}

pub fn part_1(contents: &str) -> u64 {
     contents.split("\n").map( |card| {
        let line = card.split(": ").nth(1).unwrap();
        let count = count_matches(line);
        if count > 0 { 1 << count - 1 } else { 0 }
    }).sum()
}

pub fn part_2(contents: &str) -> u64 {
    let mut count_map: HashMap<usize, u64> = HashMap::new();
    for (i, line) in contents.split("\n").enumerate() {
        let mut card = line.split(": ");
        let card_numbers = card.nth(1).unwrap();
        let matches = count_matches(card_numbers);
        count_map.insert(i + 1, matches);
    }

    let card_count = count_map.len();
    let mut cards_in_set: HashMap<usize, u64> = HashMap::new();
    for i in 1..=count_map.len() {
        match cards_in_set.get(&i) {
            Some(val) => cards_in_set.insert(i, val + 1),
            None => cards_in_set.insert(i, 1)
        };
        let current_count = cards_in_set[&i];
        let max = ((i + count_map[&i] as usize)).min(card_count);
        for j in (i + 1)..=max {
            match cards_in_set.get(&j) {
                Some(val) => cards_in_set.insert(j, val + current_count),
                None => cards_in_set.insert(j, current_count)
            };
        }
    }
    cards_in_set.values().sum()
}