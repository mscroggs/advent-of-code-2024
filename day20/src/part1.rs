use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut walls = vec![];
    let mut start = (-1, -1);
    let mut end = (-1, -1);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            let pos = (i as i64, j as i64);
            match c {
                '#' => walls.push(pos),
                'S' => start = pos,
                'E' => end = pos,
                '.' => {}
                _ => {
                    panic!();
                }
            }
        }
    }

    let mut route = vec![start];
    while !route.contains(&end) {
        let mut found = false;
        let mut next = (-1, -1);
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pos = (
                route[route.len() - 1].0 + d.0,
                route[route.len() - 1].1 + d.1,
            );
            if !walls.contains(&pos) && !route.contains(&pos) {
                assert!(!found);
                found = true;
                next = pos;
            }
        }
        route.push(next);
    }

    #[cfg(feature = "test_input")]
    let mut counts = HashMap::new();
    let mut result = 0;

    for (i, start) in route.iter().enumerate() {
        for (j, end) in route.iter().enumerate().skip(i) {
            if (start.0 - end.0).abs() + (start.1 - end.1).abs() == 2 {
                let len = j - i - 2;
                #[cfg(feature = "test_input")]
                {
                    *counts.entry(len).or_insert(0) += 1;
                }
                if len >= 100 {
                    result += 1;
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    {
        assert_eq!(counts[&2], 14);
        assert_eq!(counts[&4], 14);
        assert_eq!(counts[&6], 2);
        assert_eq!(counts[&8], 4);
        assert_eq!(counts[&10], 2);
        assert_eq!(counts[&12], 3);
        assert_eq!(counts[&20], 1);
        assert_eq!(counts[&36], 1);
        assert_eq!(counts[&38], 1);
        assert_eq!(counts[&40], 1);
        assert_eq!(counts[&64], 1);
    }
    println!("{result}");
}
