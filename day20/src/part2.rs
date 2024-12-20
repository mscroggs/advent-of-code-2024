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
            let cheat_len = (start.0 - end.0).abs() + (start.1 - end.1).abs();
            if cheat_len <= 20 {
                let len = (j - i) as i64 - cheat_len;
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
        assert_eq!(counts[&50], 32);
        assert_eq!(counts[&52], 31);
        assert_eq!(counts[&54], 29);
        assert_eq!(counts[&56], 39);
        assert_eq!(counts[&58], 25);
        assert_eq!(counts[&60], 23);
        assert_eq!(counts[&62], 20);
        assert_eq!(counts[&64], 19);
        assert_eq!(counts[&66], 12);
        assert_eq!(counts[&68], 14);
        assert_eq!(counts[&70], 12);
        assert_eq!(counts[&72], 22);
        assert_eq!(counts[&74], 4);
        assert_eq!(counts[&76], 3);
    }
    println!("{result}");
}
