use std::fs;

fn count_repeats(hand: &[u8; 5]) -> Vec<[u8; 2]> {
    let mut counter: Vec<[u8; 2]> = Vec::new();
    for &value in hand {
        if let Some(item) = counter.iter_mut().find(|item| item[0] == value) {
            item[1] += 1;
        } else {
            counter.push([value, 1]);
        }
    }
    counter
}

fn type_of_hand(count: &Vec<[u8; 2]>) -> u8 {
    // 6 -> Five of Kind
    // 5 -> Four of Kind
    // 4 -> Full House
    // 3 -> Three of Kind
    // 2 -> Two Pair
    // 1 -> One Pair
    // 0 -> High Card

    if count.len() == 1 {
        6
    } else if count.len() == 2 {
        if let Some(_item) = count.iter().find(|item| item[1] == 4) {
            5
        } else {
            4
        }
    } else if count.len() == 3 {
        if let Some(_item) = count.iter().find(|item| item[1] == 3) {
            3
        } else {
            2
        }
    } else if let Some(_item) = count.iter().find(|item| item[1] == 2) {
        1
    } else {
        0
    }
}

fn rank_cards(everything: &Vec<([u8; 5], u8, u32)>) -> u32 {
    // A vector of Hand, Hand Type, Bids
    1
}

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d7-camel-cards/sample";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    let lines = contents.lines();
    let mut _rank: Vec<u32> = Vec::new();
    // let mut hand = Hand::new();
    let mut hand = [0, 0, 0, 0, 0];
    // let mut everything: Vec<([u8; 5], u8, u32)> = Vec::new();
    // Vec of Hand, Hand Type, Bid

    let mut hand_types: Vec<Vec<([u8; 5], u32)>> =
        vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    for line in lines {
        let (cards, bids) = line.split_once(' ').unwrap();
        for (i, card) in cards.chars().enumerate() {
            hand[i] = match card {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => {
                    println!("Invalid Input");
                    0
                }
            };
        }
        let count = count_repeats(&hand);
        let hand_type = type_of_hand(&count);
        print!("{:?}, ", hand_type);
        if let Some(vec) = hand_types.get_mut(hand_type as usize) {
            vec.push((hand, bids.parse().unwrap()))
        };
    }

    println!("{:?}", hand_types;
}
