// use std::env;
use std::fmt;
use std::fs;
// use std::path::Path;

struct Pos {
    x: u32,
    y: u32,
    id: u32,
}

struct ID {
    id: u32,
    len: u32,
    val: u32,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Pos {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ID: {}, x: {}, y: {}", self.id, self.x, self.y)
    }
}

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d3-gear-ratios/input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let mut num_index: Vec<u32> = Vec::new();
    let mut sym_index: Vec<u32> = Vec::new();
    // let mut num_coord: Vec<Pos> = Vec::new();
    // let mut sym_coord: Vec<Pos> = Vec::new();

    let line: String = contents.split('\n').map(|s| s.to_string()).collect();
    let width: u32 = (line.len() as f64).sqrt() as u32;

    // println!("{line}");

    for (i, elm) in line.chars().enumerate() {
        if elm.is_ascii_punctuation() && elm != '.' {
            sym_index.push(i as u32);
        };
        if elm.is_numeric() {
            num_index.push(i as u32);
        };
    }

    let num_coord: Vec<Pos> = position(num_index, width);
    let sym_coord: Vec<Pos> = position(sym_index, width);

    // for elm in &sym_coord {
    //     println!("{}", elm);
    // }

    let (num_id, _sym_id) = make_id(&line);

    // for elm in &num_id {
    //     println!("ID {}, Length {}, Value: {}", elm.id, elm.len, elm.val);
    // }

    let ids = id_match(&num_coord, &sym_coord);

    let solution = id_val(&ids, &num_id);
    println!("Solution: {solution}");
}

fn id_val(ids: &Vec<(u32, u32)>, vals: &Vec<ID>) -> u32 {
    let mut holder: Vec<(u32, u32)> = Vec::new();

    for val in vals {
        'outer: for id in ids {
            let matchers = val.id..(val.id + val.len);
            for i in matchers {
                if i == id.0 {
                    // println!("Matched: {}, num matched", val.val);
                    holder.push((val.val, id.1));
                    break 'outer;
                }
            }
        }
    }
    two_id(&holder).iter().sum()
}

fn position(sym_index: Vec<u32>, width: u32) -> Vec<Pos> {
    let mut vec: Vec<Pos> = Vec::new();
    for index in sym_index {
        let x = index % width;
        let y = index / width;
        vec.push(Pos { x, y, id: index });
    }
    vec
}

fn make_id(contents: &str) -> (Vec<ID>, Vec<ID>) {
    let mut num: Vec<ID> = Vec::new();
    let mut sym: Vec<ID> = Vec::new();

    // Numbers
    let v: Vec<&str> = contents.split(|c: char| !c.is_numeric()).collect();

    let mut j: u32 = 0;
    for elm in v.iter() {
        if let Ok(parsed_val) = elm.parse::<u32>() {
            num.push(ID {
                id: j,
                len: elm.len() as u32,
                val: parsed_val,
            });
            // println!("{elm}, ID {j}");
            j += elm.len() as u32;
        }
        j += 1;
    }

    // Symbols
    let w: Vec<&str> = contents
        .split(|c: char| !c.is_ascii_punctuation() || c == '.')
        .collect();

    let mut j: u32 = 0;
    for &elm in w.iter() {
        if !elm.is_empty() {
            sym.push(ID {
                id: j,
                len: elm.len() as u32,
                val: 0,
            });
            // println!("{elm}, ID: {j}");
            j += elm.len() as u32;
        }
        j += 1;
    }

    (num, sym)
}

fn id_match(num_coord: &Vec<Pos>, sym_coord: &Vec<Pos>) -> Vec<(u32, u32)> {
    let mut match_num: Vec<(u32, u32)> = Vec::new();

    for sym in sym_coord {
        let pos_sym = walk(sym);
        for wal in pos_sym {
            for pla in num_coord {
                if wal == (pla.x as i32, pla.y as i32) {
                    // println!("Match: {}, {}, ID: {}", wal.0, wal.1, pla.id);
                    match_num.push((pla.id, sym.id));
                }
            }
        }
    }
    match_num
}

fn walk(sym_coord: &Pos) -> [(i32, i32); 8] {
    let (x, y) = (sym_coord.x as i32, sym_coord.y as i32);

    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
    ]
}

fn two_id(input: &[(u32, u32)]) -> Vec<u32> {
    let mut count_map = std::collections::HashMap::new();

    // Step 1: Count occurrences of counts
    for &(value, count) in input {
        let counter = count_map.entry(count).or_insert(Vec::new());
        counter.push(value);
    }

    // Step 2: Multiply pairs with the same count
    let mut result = Vec::new();
    for values in count_map.values() {
        if values.len() == 2 {
            result.push(values[0] * values[1]);
        }
    }

    result
}
