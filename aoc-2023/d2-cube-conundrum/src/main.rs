use std::fmt;
use std::fs;

struct Games {
    red: u32,
    green: u32,
    blue: u32,
}

impl Games {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Games {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "Red: {}, Green: {}, Blue {}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d2-cube-conundrum/input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{contents}");

    // rules, max Red/Greem/Blue
    // let rules = [12, 13, 14];
    let _matcher = ["red", "green", "blue"];

    let lines = contents.lines();
    let mut solution: usize = 0;

    for (i, line) in lines.enumerate() {
        let (game_name, data) = line.split_once(": ").unwrap();

        // let mut is_possible: bool = true;
        let mut power = Games::new();

        for sub_game in data.split("; ") {
            for values in sub_game.split(", ") {
                let (num, color) = values.split_once(" ").unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => {
                        if num > power.red {
                            // println!("Game {game_name} is not possible");
                            // is_possible = false;
                            power.red = num;
                        }
                    }
                    "green" => {
                        if num > power.green {
                            // println!("Game {game_name} is not possible");
                            // is_possible = false;
                            power.green = num;
                        }
                    }
                    "blue" => {
                        if num > power.blue {
                            // println!("Game {game_name} is not possible");
                            // is_possible = false;
                            power.blue = num;
                        }
                    }
                    _ => {
                        println!("You have an error");
                    }
                }
            }
        }
        // if is_possible {
        //     println!("Game {game_name} is possible");
        //     solution += i + 1;
        // } else {
        //     println!("Game {game_name} is not possible");
        // }

        println!("Game is {power} with a power of {}", power.power());
        solution += power.power() as usize;
    }

    println!("Solution {solution}");
}
