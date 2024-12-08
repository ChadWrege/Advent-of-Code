use std::fs;

fn main() {
    let file_path = "/home/chadw/Repos/aoc-2023/d6-wait-for-it/input";
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    // let times = Vec::new();
    // let dist = Vec::new();

    let (times, dist) = contents.split_once("\n").unwrap();
    let (_, times) = times.split_once(":").unwrap();
    let (_, dist) = dist.split_once(":").unwrap();

    let times: Vec<u64> = times
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let dist: Vec<u64> = dist
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut counter: Vec<i64> = Vec::new();

    let times = vec![concatenate_numbers(times)];
    let dist = vec![concatenate_numbers(dist)];

    for (i, time) in times.iter().enumerate() {
        // println!("{time}, and {}", dist[i]);
        let mut go_down = time / 2;
        let mut go_up = 0;
        counter.push(0);

        if time % 2 == 0 {
            go_up = time / 2;
            counter[i] -= 1;
        } else {
            go_up = time / 2 + 1;
        }

        let mut my_dist = go_down * go_up;
        // println!("{my_dist}");
        while my_dist > dist[i] {
            go_up += 1;
            go_down -= 1;
            my_dist = go_up * go_down;
            counter[i] += 2;
        }
    }
    println!("{:?}", counter);
    let sol: i64 = counter.iter().product();
    println!("{sol}");
}

fn concatenate_numbers(vec: Vec<u64>) -> u64 {
    let concatenated_string: String = vec.into_iter().map(|x| x.to_string()).collect();
    let parsed_number: u64 = concatenated_string.parse().unwrap();
    parsed_number
}
