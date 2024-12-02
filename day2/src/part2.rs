use itertools::izip;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn passing(n: &[i32]) -> bool {
    if n[0] < n[1] {
        for (i, j) in izip!(&n[0..n.len() - 1], &n[1..]) {
            if i >= j || j - i > 3 {
                return false;
            }
        }
        true
    } else {
        for (i, j) in izip!(&n[0..n.len() - 1], &n[1..]) {
            if i <= j || i - j > 3 {
                return false;
            }
        }
        true
    }
}

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

        if passing(&numbers) {
            count += 1;
        } else {
            let mut sub_n = numbers[1..].to_vec();
            for i in 0..numbers.len() {
                if passing(&sub_n) {
                    count += 1;
                    break;
                }
                if i < numbers.len() - 1 {
                    sub_n[i] = numbers[i];
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(count, 4);

    println!("{count}");
}
