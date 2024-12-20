use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn possible<'a>(pattern: &'a str, available: &[&str], cache: &mut HashMap<&'a str, bool>) -> bool {
    if pattern.is_empty() {
        true
    } else if cache.contains_key(pattern) {
        cache[pattern]
    } else {
        for a in available {
            if let Some(new_pattern) = pattern.strip_prefix(a) {
                if possible(new_pattern, available, cache) {
                    cache.insert(pattern, true);
                    return true;
                }
            }
        }
        cache.insert(pattern, false);
        false
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
        .filter(|i| possible(i, &available, &mut cache))
        .count();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 6);

    println!("{result}");
}
