use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = vec![];
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col, char) in line.chars().enumerate() {
            let pos = (row as i32, col as i32);
            if char == '#' {
                walls.push(pos);
            } else if char == 'E' {
                end = pos;
            } else if char == 'S' {
                start = pos;
            } else {
                assert_eq!(char, '.');
            }
        }
    }

    let mut shortest = HashMap::new();
    let mut next = vec![];
    shortest.insert((start, (0, 1)), 0);
    next.push((start, (0, 1)));

    loop {
        let distance = shortest[&next[0]];
        for pos in &next {
            let moved = ((pos.0 .0 + pos.1 .0, pos.0 .1 + pos.1 .1), pos.1);
            if !walls.contains(&moved.0) {
                let value = *shortest.entry(moved).or_insert_with(|| distance + 1);
                *shortest.get_mut(&moved).unwrap() = min(value, distance + 1)
            }
            for d in if pos.1 == (1, 0) || pos.1 == (-1, 0) {
                [(0, 1), (0, -1)]
            } else {
                [(1, 0), (-1, 0)]
            } {
                let value = *shortest
                    .entry((pos.0, d))
                    .or_insert_with(|| distance + 1000);
                *shortest.get_mut(&(pos.0, d)).unwrap() = min(value, distance + 1000)
            }
        }

        if let Some(&next_distance) = shortest.values().filter(|i| **i > distance).min() {
            next = shortest
                .iter()
                .filter(|(_, i)| **i == next_distance)
                .map(|(i, _)| *i)
                .collect::<Vec<_>>();
        } else {
            break;
        }
    }

    let result = min(
        min(shortest[&(end, (1, 0))], shortest[&(end, (-1, 0))]),
        min(shortest[&(end, (0, 1))], shortest[&(end, (0, -1))]),
    );

    #[cfg(feature = "test_input")]
    assert_eq!(result, 11048);

    println!("{result}");
}
