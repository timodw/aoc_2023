use std::collections::HashMap;

fn check_surroundings(row: usize, col: usize, grid: &Vec<&[u8]>, check_gears: bool) -> (bool, (usize, usize)) {
    let row = row as i64;
    let col = col as i64;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; };
            if row + i >= 0 && row + i < grid.len() as i64 &&
               col + j >= 0 && col + j < grid[0].len() as i64 {
                let c = grid[(row + i) as usize][(col + j) as usize] as char;
                if ! c.is_numeric() && c != '.' && (!check_gears || c == '*') {
                    return (true, ((row + i) as usize, (col + j) as usize));
                }
            }
        }
    }
    (false, (0, 0))
}

fn make_grid(contents: &str) -> Vec<&[u8]> {
    let mut grid: Vec<&[u8]> = Vec::new();
    for line in contents.split("\n") {
        grid.push(line.as_bytes());
    };
    grid
}

pub fn part_1(contents: &str) -> u64 {
    let grid = make_grid(contents);

    let mut sum = 0u64;
    for (i, row) in grid.iter().enumerate() {
        let mut number_str: String = String::from("");
        let mut valid_number = false;
        for (j, c) in row.iter().enumerate() {
            if (*c as char).is_numeric() {
                number_str.push(*c as char);
                if check_surroundings(i, j, &grid, false).0 {
                    valid_number = true
                };
            } else if !number_str.is_empty()  {
                if valid_number {
                    sum += number_str.parse::<u64>().unwrap();
                    valid_number = false;
                }
                number_str = String::from("");
            }
        }
        if !number_str.is_empty() && valid_number {
            sum += number_str.parse::<u64>().unwrap();
        }
    }
    sum
}

pub fn part_2(contents: &str) -> u64 {
    let grid = make_grid(contents);
    let mut gears:HashMap<(usize, usize), u64> = HashMap::new();
    let mut sum = 0u64;
    for (i, row) in grid.iter().enumerate() {
        let mut number_str: String = String::from("");
        let mut valid_number = false;
        let mut current_gear = (0usize, 0usize);
        for (j, c) in row.iter().enumerate() {
            if (*c as char).is_numeric() {
                number_str.push(*c as char);
                let res = check_surroundings(i, j, &grid, true); 
                if res.0 {
                    valid_number = true;
                    current_gear = res.1;
                }
            } else if !number_str.is_empty()  {
                if valid_number {
                    let number = number_str.parse::<u64>().unwrap();
                    match gears.get(&current_gear) {
                        Some(n) => {
                            sum += number * *n 
                        },
                        None => { gears.insert(current_gear, number); }
                    }
                    valid_number = false;
                }
                number_str = String::from("");
            }
        }
        if !number_str.is_empty() && valid_number {
            let number = number_str.parse::<u64>().unwrap();
            match gears.get(&current_gear) {
                Some(n) => { sum += number * *n },
                None => { gears.insert(current_gear, number); }
            }
        }
    }
    sum
}