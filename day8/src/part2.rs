use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else if a < b {
        hcf(b, a)
    } else {
        hcf(b, a % b)
    }
}

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
                ants.entry(c)
                    .or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
            cols = j as i32 + 1;
        }
        rows = i as i32 + 1;
    }

    let mut nodes = vec![];
    for a in ants.values() {
        for (i, a_i) in a.iter().enumerate() {
            for (j, a_j) in a.iter().enumerate() {
                if i != j {
                    for jump in {
                        let jump = (a_i.0 - a_j.0, a_i.1 - a_j.1);
                        let h = hcf(jump.0.abs(), jump.1.abs());
                        let jump = (jump.0 / h, jump.1 / h);
                        [jump, (-jump.0, -jump.1)]
                    } {
                        let mut pos = (a_i.0, a_i.1);
                        while pos.0 >= 0 && pos.1 >= 0 && pos.0 < rows && pos.1 < cols {
                            if !nodes.contains(&pos) {
                                nodes.push(pos);
                            }
                            pos.0 += jump.0;
                            pos.1 += jump.1;
                        }
                    }
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(nodes.len(), 34);

    println!("{}", nodes.len());
}
