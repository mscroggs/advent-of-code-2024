use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut number = line.parse::<u64>().unwrap();

        for _ in 0..2000 {
            number ^= number * 64;
            number %= 16777216;

            number ^= number / 32;
            number %= 16777216;

            number ^= number * 2048;
            number %= 16777216;
        }
        result += number;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 37327623);

    println!("{result}");
}
