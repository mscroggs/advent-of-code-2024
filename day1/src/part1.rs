use itertools::izip;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut first = vec![];
    let mut second = vec![];
    for line in reader.lines() {
        let numbers = line
            .unwrap()
            .split("   ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();

    let result = izip!(&first, &second)
        .map(|(i, j)| (i - j).abs())
        .sum::<i32>();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 11);

    println!("{result}");
}
