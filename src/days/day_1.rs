pub fn part_1(contents: &str) -> i64 {
    contents.split("\n").map(|calibration_str| {
        let digits: Vec<char> = calibration_str.chars()
                     .filter(|c| c.is_numeric()).collect();
        format!("{}{}", digits[0], digits[digits.len() - 1])
        .parse::<i64>().unwrap()
    }).sum::<i64>()
}

pub fn part_2(contents: &str) -> i64 {
    contents.split("\n").map(|calibration_str| {
        let digits: Vec<char> = (0..calibration_str.as_bytes().len()).map(|i| {
            match &calibration_str[i..] {
                s if s.chars().next().unwrap().is_numeric() => s.chars().next().unwrap(),
                s if s.starts_with("one") => '1',
                s if s.starts_with("two") => '2',
                s if s.starts_with("three") => '3',
                s if s.starts_with("four") => '4',
                s if s.starts_with("five") => '5',
                s if s.starts_with("six") => '6',
                s if s.starts_with("seven") => '7',
                s if s.starts_with("eight") => '8',
                s if s.starts_with("nine") => '9',
                _ => '_'
            }
        }).filter(|c| c.is_numeric()).collect();
        format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<i64>().unwrap()
    }).sum()
}