use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);

    let re = Regex::new(r"mul\(([1-9][0-9]*),([1-9][0-9]*)\)").unwrap();

    let mut total = 0;
    for i in re.captures_iter(&data) {
        let (_, numbers) = i.extract::<2>();
        let n = numbers[0].parse::<i32>().unwrap();
        let m = numbers[1].parse::<i32>().unwrap();
        total += n * m;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 161);

    println!("{total}");
}
