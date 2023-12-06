pub fn part_1(contents: &str) -> u64 {
    let mut lines = contents.split("\n");
    let times: Vec<f64> = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|n| {
        n.parse().unwrap()
    }).collect();
    let records: Vec<f64> = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|n| {
        n.parse().unwrap()
    }).collect();
    times.iter().zip(records.iter()).map(|(t, r)| {
        let x_0 = (-t + (t.powf(2.) - 4. * r).sqrt()) / (-2.);
        let x_1 = (-t - (t.powf(2.) - 4. * r).sqrt()) / (-2.);
        x_1.ceil() as u64 - x_0.floor() as u64 - 1
    }).product()
}

pub fn part_2(contents: &str) -> u64 {
    let mut lines = contents.split("\n");
    let t: f64 = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<String>().parse().unwrap();
    let r: f64 = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<String>().parse().unwrap();
    let x_0 = (-t + (t.powf(2.) - 4. * r).sqrt()) / (-2.);
    let x_1 = (-t - (t.powf(2.) - 4. * r).sqrt()) / (-2.);
    x_1.ceil() as u64 - x_0.floor() as u64 - 1
}