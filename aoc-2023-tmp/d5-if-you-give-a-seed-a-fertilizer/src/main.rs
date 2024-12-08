use std::fs;
// use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Reng {
    start: i64,
    len: i64,
    end: i64,
}

#[derive(Debug, Copy, Clone)]
struct Mapper {
    source: i64,
    destination: i64,
    len: i64,
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

impl Reng {
    fn new_len(start: i64, len: i64) -> Reng {
        let end = start + len - 1;
        Reng { start, len, end }
    }
    fn new(start: i64, end: i64) -> Reng {
        let len = end - start + 1;
        Reng { start, len, end }
    }
}

impl Ord for Reng {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Reng {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mapper {
    fn map(&self) -> i64 {
        self.destination - self.source
    }
    fn end(&self) -> i64 {
        self.source + self.len - 1
    }
}

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d5-if-you-give-a-seed-a-fertilizer/input";
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    let input: Vec<&str> = contents.split("\n\n").collect();
    let mut range_seeds: Vec<Reng> = Vec::new();

    for (i, paragraph) in input.iter().enumerate() {
        if i == 0 {
            let (_, seed) = paragraph.split_once(':').unwrap();
            let seeds_str: Vec<&str> = seed.split_whitespace().collect();

            for (i, elm) in seeds_str.iter().enumerate() {
                if i % 2 == 0 {
                    range_seeds.push(Reng::new_len(
                        elm.parse().unwrap(),
                        seeds_str[i + 1].parse().unwrap(),
                    ));
                }
            }
            // println!("These are our seeds {:?}", range_seeds);
        } else {
            let mut maps: Vec<Mapper> = Vec::new();
            for (j, line) in paragraph.lines().enumerate() {
                if j == 0 {
                    continue;
                } else {
                    let map_str: Vec<&str> = line.split_whitespace().collect();
                    let mut map: Vec<i64> = Vec::new();
                    for elm in map_str {
                        map.push(elm.parse().unwrap());
                    }
                    maps.push(Mapper {
                        source: map[1],
                        destination: map[0],
                        len: map[2],
                    });
                }
            }
            range_seeds = range_mapper(&range_seeds, &maps);
        }
    }
    // println!("The Last Ranges {:?}", range_seeds);
    println!("The Final Solution: {:?}", range_seeds.iter().min());
}

fn range_mapper(range_seeds: &[Reng], maps: &[Mapper]) -> Vec<Reng> {
    let mut output: Vec<Reng> = Vec::new();

    for seeds in range_seeds {
        let mut check_again: Vec<Reng> = Vec::new();
        check_again.push(*seeds);

        while let Some(mut check) = check_again.pop() {
            let mut in_range = false;

            for map in maps {
                let (seed_start, seed_end) = (check.start, check.end);

                match (
                    seed_start >= map.source && seed_end <= map.end(),
                    seed_start >= map.source && seed_start <= map.end(),
                    seed_end >= map.source && seed_end <= map.end(),
                ) {
                    (true, _, _) => {
                        // All Seeds Mapped
                        check.start += map.map();
                        check.end += map.map();
                        output.push(check);
                        in_range = true;
                        break;
                    }
                    (false, true, _) => {
                        // Seed Start to Intersection End: Mapped
                        check.start += map.map();
                        check.end = map.end() + map.map();
                        output.push(check);
                        // Intersection End + 1 to Seed End not mapped
                        check_again.push(Reng::new(map.end() + 1, seed_end));
                        in_range = true;
                        break;
                    }
                    (false, false, true) => {
                        // Intersection to Seed End: Mapped
                        check.start = map.source + map.map();
                        check.end += map.map();
                        output.push(check);
                        // Seed Start to Map Start, not mapped
                        check_again.push(Reng::new(seed_start, map.source - 1));
                        in_range = true;
                        break;
                    }
                    _ => (),
                }
            }

            if !in_range {
                output.push(check);
            }
        }
    }

    output
}
