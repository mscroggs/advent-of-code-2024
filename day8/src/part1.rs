use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut ants = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                ants.entry(c).or_insert_with(Vec::new).push((i, j));
            }
            cols = j + 1;
        }
        rows = i + 1;
    }

    let mut nodes = vec![];
    for a in ants.values() {
        for (i, a_i) in a.iter().enumerate() {
            for (j, a_j) in a.iter().enumerate() {
                if i != j && 2 * a_i.0 >= a_j.0 && 2 * a_i.1 >= a_j.1 {
                    let node = (2 * a_i.0 - a_j.0, 2 * a_i.1 - a_j.1);
                    if node.0 < rows && node.1 < cols && !nodes.contains(&node) {
                        nodes.push(node);
                    }
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(nodes.len(), 14);

    println!("{}", nodes.len());
}
