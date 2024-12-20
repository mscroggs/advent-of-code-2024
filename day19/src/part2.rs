use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn ways<'a>(pattern: &'a str, available: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.is_empty() {
        1
    } else if cache.contains_key(pattern) {
        cache[pattern]
    } else {
        let count = available
            .iter()
            .filter(|a| pattern.starts_with(*a))
            .map(|a| ways(&pattern[a.len()..], available, cache))
            .sum::<usize>();
        cache.insert(pattern, count);
        count
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let available = lines[0].split(", ").collect::<Vec<_>>();

    let mut cache = HashMap::new();

    let result = lines
        .iter()
        .skip(2)
        .map(|i| ways(i, &available, &mut cache))
        .sum::<usize>();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 16);

    println!("{result}");
}
