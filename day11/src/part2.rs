use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut stones = HashMap::new();
    for stone in reader.lines().map(|i| i.unwrap()).collect::<Vec<_>>()[0]
        .split(" ")
        .map(|i| i.parse::<i64>().unwrap())
    {
        *stones.entry(stone).or_insert_with(|| 0) += 1;
    }

    #[cfg(feature = "test_input")]
    let nblinks = 25;
    #[cfg(not(feature = "test_input"))]
    let nblinks = 75;

    for _ in 0..nblinks {
        let mut new_stones = HashMap::new();
        for (stone, count) in &stones {
            if *stone == 0 {
                *new_stones.entry(1).or_insert_with(|| 0) += count;
            } else if format!("{stone}").len() % 2 == 0 {
                let s = format!("{stone}");
                let len = s.len();
                *new_stones
                    .entry(s[..len / 2].parse::<i64>().unwrap())
                    .or_insert_with(|| 0) += count;
                *new_stones
                    .entry(s[len / 2..].parse::<i64>().unwrap())
                    .or_insert_with(|| 0) += count;
            } else {
                *new_stones.entry(*stone * 2024).or_insert_with(|| 0) += count;
            }
        }
        stones = new_stones;
    }

    let total = stones.values().sum::<i64>();

    #[cfg(feature = "test_input")]
    assert_eq!(total, 55312);

    println!("{total}");
}
