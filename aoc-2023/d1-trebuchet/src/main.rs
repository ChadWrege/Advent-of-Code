use std::fmt;
use std::fs;

struct ID {
    id: usize,
    val: u32,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for ID {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ID: {}, Val: {}", self.id, self.val)
    }
}

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d1-trebuchet/input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{contents}");
    let lines = contents.lines();

    let dic: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut val: u32 = 0;

    for elm in lines {
        let (rr, ll) = matching(&dic, elm);

        // println!("Left is {ll}");
        // println!("Right Is {rr}");

        let mut vec: Vec<ID> = Vec::new();

        for (i, elm2) in elm.chars().enumerate() {
            if elm2.is_numeric() {
                let num = elm2.to_digit(10).unwrap();
                // println!("{num} ID {i}");
                let push = ID { id: i, val: num };
                vec.push(push);
            }
        }

        let lll = vec.first().unwrap_or(&ID { id: 1000, val: 0 });
        let rrr = vec.last().unwrap_or(&ID { id: 0, val: 0 });

        let mut first = lll.val;
        let mut last = rrr.val;

        if ll.id < lll.id {
            first = ll.val;
        }

        if rr.id > rrr.id {
            last = rr.val;
        }

        val += first * 10 + last;
        // println!(" Val is {val}, First is {first}, last is {last}")
    }

    println!("The Val is {}", val);
}

fn matching(dic: &[(&str, u32); 9], lines: &str) -> (ID, ID) {
    let mut len = 0;
    let mut id_left = 1000000;
    let mut id_right = 0;
    let mut val_left = 0;
    let mut val_right = 0;

    for elm in *dic {
        let left = lines.find(elm.0).unwrap_or(1000000);
        let right = lines.rfind(elm.0).unwrap_or(0);

        if left < id_left {
            id_left = left;
            val_left = elm.1;
        }

        if right > id_right {
            id_right = right;
            val_right = elm.1;
            len = elm.0.len() - 1;
        }
    }

    let left = ID {
        id: id_left,
        val: val_left,
    };
    let right = ID {
        id: id_right + len,
        val: val_right,
    };

    (right, left)
}
