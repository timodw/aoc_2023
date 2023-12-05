use rayon::prelude::*;

fn parse_range(line: &str) -> (u64, u64, u64) {
    let mut numbers = line.split_whitespace();
    (
        numbers.next().unwrap().parse().unwrap(),
        numbers.next().unwrap().parse().unwrap(),
        numbers.next().unwrap().parse().unwrap()
    )
}

pub fn part_1(contents: &str) -> u64 {
    let mut map_split = contents.split("\n\n");
    let seeds: Vec<u64> = map_split.next().unwrap()
                                   .split(": ").last().unwrap()
                                   .split_whitespace().map(|seed_id| {seed_id.parse::<u64>().unwrap()}).collect();

    let mappings_vec: Vec<Vec<(u64, u64, u64)>> = map_split.map(|range_map| {
        range_map.split("\n").skip(1).map(|l| parse_range(l)).collect()
    }).collect(); 

    seeds.iter().map(|s| {
        let mut s = *s;
        for range_map in mappings_vec.iter() {
            for range in range_map.iter() {
                if range.1 <= s && s < range.1 + range.2 {
                    s = s - range.1 + range.0;
                    break;
                }
            }
        }
        s
    }).min().unwrap()
}

pub fn part_2(contents: &str) -> u64 {
    let mut map_split = contents.split("\n\n");
    let seeds: Vec<u64> = map_split.next().unwrap()
                                   .split(": ").last().unwrap()
                                   .split_whitespace().map(|seed_id| {seed_id.parse::<u64>().unwrap()}).collect();

    let mappings_vec: Vec<Vec<(u64, u64, u64)>> = map_split.map(|range_map| {
        range_map.split("\n").skip(1).map(|l| parse_range(l)).collect()
    }).collect(); 

    seeds.chunks(2).map(|pair| {
        let start = pair[0];
        let length = pair[1];
        (start..(start + length)).into_par_iter().map(|s| {
            let mut s = s;
            for range_map in mappings_vec.iter() {
                for range in range_map.iter() {
                    if range.1 <= s && s < range.1 + range.2 {
                        s = s - range.1 + range.0;
                        break;
                    }
                }
            }
            s
        }).min().unwrap()
    }).min().unwrap()
}