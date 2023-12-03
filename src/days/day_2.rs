pub fn part_1(contents: &str) -> i64 {
    contents.split("\n").map(|line| {
        let mut splitted_game = line.split(": ");
        let game_id = splitted_game.next().unwrap().split(" ").nth(1).unwrap().parse::<i64>().unwrap();
        for sub_game in splitted_game.next().unwrap().split("; ") {
            for draw in sub_game.split(", ") {
                let mut splitted_draw = draw.split(" ");
                let count = splitted_draw.next().unwrap().parse::<i64>().unwrap();
                match splitted_draw.next().unwrap() {
                    "red" => if count > 12 { return 0; },
                    "green" => if count > 13 { return 0; },
                    "blue" => if count > 14 { return 0 },
                    _ => {}
                }
            }
        }
        game_id
    }).sum::<i64>()
}

pub fn part_2(contents: &str) -> u64 {
    contents.split("\n").map(|line| {
        let mut maximums = vec![0; 3];
        let mut splitted_game = line.split(": ");
        for sub_game in splitted_game.nth(1).unwrap().split("; ") {
            for draw in sub_game.split(", ") {
                let mut splitted_draw = draw.split(" ");
                let count = splitted_draw.next().unwrap().parse::<u64>().unwrap();
                match splitted_draw.next().unwrap() {
                    "red" => if count > maximums[0] { maximums[0] = count; },
                    "green" => if count > maximums[1] { maximums[1] = count ; },
                    "blue" => if count > maximums[2] { maximums[2] = count },
                    _ => {}
                }
            }
        }
        maximums.iter().fold(1, |acc, &c| acc * c)
    }).sum::<u64>()
}