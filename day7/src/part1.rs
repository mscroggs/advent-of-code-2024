use std::fs::File;
use std::io::{prelude::*, BufReader};

fn possible(first_number: i64, numbers: &[i64], aim: i64) -> bool {
    if numbers.is_empty() {
        first_number == aim
    } else {
        for value in [first_number + numbers[0], first_number * numbers[0]] {
            if possible(value, &numbers[1..], aim) {
                return true;
            }
        }
        false
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split(": ").collect::<Vec<_>>();
        let aim = line[0].parse::<i64>().unwrap();
        let numbers = line[1]
            .split(" ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if possible(numbers[0], &numbers[1..], aim) {
            total += aim;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 3749);

    println!("{}", total);
}
