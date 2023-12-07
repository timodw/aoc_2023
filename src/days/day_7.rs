use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    hand: Vec<u64>,
    hand_occurrences: Vec<u64>,
    bid_value: u64,
    part_2: bool,
    joker_count: u64
}

impl Hand {
    fn new(hand: Vec<u64>, bid_value: u64, part_2: bool, joker_count: u64) -> Hand {
        let mut hand_occurrences = vec![0;13];
        let diff = if part_2 { 1 } else { 2 };

        for card in hand.iter() {
            hand_occurrences[(*card - diff) as usize] += 1;
        }
        if part_2 { hand_occurrences[0] = 0; }
        Hand { hand, hand_occurrences, bid_value, part_2, joker_count }
    }
}

fn get_hand_value(counts: &Vec<u64>, part_2: bool, joker_count: u64) -> u64 {
    let mut sorted_counts = counts.clone();
    sorted_counts.sort_by(|a, b| b.cmp(a));


    let max_count = sorted_counts[0];
    let second_max_count = sorted_counts[1];

    if max_count + joker_count == 5 {
        return 6;
    } else if max_count + joker_count == 4 {
        return 5;
    } else if max_count + joker_count == 3 && second_max_count == 2 {
        return 4;
    } else if max_count + joker_count == 3 {
        return 3;
    } else if max_count + joker_count == 2 && second_max_count == 2 {
        return 2;
    } else if max_count + joker_count == 2 {
        return 1;
    }
    0
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = get_hand_value(&self.hand_occurrences, self.part_2, self.joker_count);
        let other_val = get_hand_value(&other.hand_occurrences, self.part_2, other.joker_count);

        if self_val == other_val {
            for (h1, h2) in self.hand.iter().zip(other.hand.iter()) {
                let ord = h1.cmp(&h2);
                if ord != Ordering::Equal { return ord }
            }
        }
        self_val.cmp(&other_val)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

impl Eq for Hand {}

pub fn part_1(contents: &str) -> u64 {
    let lines = contents.split("\n");
    let mut hands: Vec<Hand> = lines.map(|l| {
        let mut it = l.split_whitespace();
        let h: Vec<u64> = it.next().unwrap().chars().map(|c| {
            match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c => c.to_digit(10).unwrap() as u64
            }
        }).collect();
        let bid: u64 = it.next().unwrap().parse().unwrap();
        Hand::new(h, bid, false, 0)
    }).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| {(i as u64 + 1) * h.bid_value}).sum()
}

pub fn part_2(contents: &str) -> u64 {
    let lines = contents.split("\n");
    let mut hands: Vec<Hand> = lines.map(|l| {
        let mut it = l.split_whitespace();
        let mut joker_count = 0;
        let h: Vec<u64> = it.next().unwrap().chars().map(|c| {
            match c {
                'T' => 10,
                'J' => { joker_count+= 1; 1 },
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                c => c.to_digit(10).unwrap() as u64
            }
        }).collect();
        let bid: u64 = it.next().unwrap().parse().unwrap();
        Hand::new(h, bid, true, joker_count)
    }).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| {(i as u64 + 1) * h.bid_value}).sum()
}