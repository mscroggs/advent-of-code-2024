use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input_part2").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut all_prices = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut number = line.parse::<i64>().unwrap();

        let mut seq = (10, 10, 10, 10);

        let mut prices = HashMap::new();

        let mut prev = number % 10;

        for _ in 0..2000 {
            number ^= number * 64;
            number %= 16777216;

            number ^= number / 32;
            number %= 16777216;

            number ^= number * 2048;
            number %= 16777216;

            let n = number % 10;

            seq.0 = seq.1;
            seq.1 = seq.2;
            seq.2 = seq.3;
            seq.3 = n - prev;

            if seq.0 != 10 {
                prices.entry(seq).or_insert(n);
            }
            prev = n;
        }

        for (i, j) in prices.iter() {
            *all_prices.entry(*i).or_insert(0) += *j;
        }
    }

    let result = *all_prices.values().max().unwrap();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 23);

    println!("{result}");
}
