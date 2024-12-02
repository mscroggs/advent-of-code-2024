use itertools::izip;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let numbers = line
            .unwrap()
            .split(" ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let numbers = if numbers[0] > numbers[1] {
            numbers.iter().map(|i| -i).collect::<Vec<_>>()
        } else {
            numbers
        };
        let mut passing = true;
        for (i, j) in izip!(&numbers[0..numbers.len() - 1], &numbers[1..]) {
            if i >= j || j - i > 3 {
                passing = false;
                break;
            }
        }
        if passing {
            count += 1;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(count, 2);

    println!("{count}");
}
