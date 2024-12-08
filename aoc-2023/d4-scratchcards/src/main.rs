// use std::collections::HashMap;
// use std::fmt;
use std::fs;

#[derive(Debug)]
struct Cards {
    card: usize,
    matches: u32,
    copies: u32,
}
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// impl fmt::Display for Pos {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "ID: {}, x: {}, y: {}", self.id, self.x, self.y)
//     }
// }

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d4-scratchcards/input";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    let mut solution = 0;
    // let mut num_cards = HashMap::new();
    let mut cards: Vec<Cards> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        cards.push(Cards {
            card: i,
            matches: 0,
            copies: 1,
        });

        let (_card, num) = line.split_once(':').unwrap();
        let (ans, sol) = num.split_once('|').unwrap();

        for loser in ans.split_whitespace() {
            for winner in sol.split_whitespace() {
                if winner == loser {
                    cards[i].matches += 1;

                    // println!("{card} is a winner with {} matches", num_cards[i].matches);
                }
            }
        }

        // if win_count > 0 {
        //     solution += 2_i32.pow(win_count - 1);
        // }
        // println!("{card} won with {win_count} matches, total: {solution}");
    }

    for i in 0..cards.len() {
        for j in (i + 1)..(i + 1 + cards[i].matches as usize) {
            cards[j].copies += cards[i].copies;
        }
    }

    for card in &cards {
        solution += card.copies;
    }
    println!("{:?}", cards);
    println!("{solution}")
}
