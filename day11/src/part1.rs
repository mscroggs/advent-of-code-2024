use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut stones = reader.lines().map(|i| i.unwrap()).collect::<Vec<_>>()[0]
        .split(" ")
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in &stones {
            if *stone == 0 {
                new_stones.push(1);
            } else if format!("{stone}").len() % 2 == 0 {
                let s = format!("{stone}");
                let len = s.len();
                new_stones.push(s[..len / 2].parse::<i64>().unwrap());
                new_stones.push(s[len / 2..].parse::<i64>().unwrap());
            } else {
                new_stones.push(*stone * 2024);
            }
        }
        stones = new_stones;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(stones.len(), 55312);

    println!("{}", stones.len());
}
