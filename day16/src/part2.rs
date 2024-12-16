use std::cmp::min;
use std::cmp::Ordering;
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
    shortest.insert((start, (0, 1)), (0, vec![(0, 0)]));
    next.push((start, (0, 1)));

    loop {
        let distance = shortest[&next[0]].0;
        for pos in &next {
            let moved = ((pos.0 .0 + pos.1 .0, pos.0 .1 + pos.1 .1), pos.1);
            let path = shortest[pos].1.clone();
            let mut path2 = path.clone();
            path2.push(moved.0);
            if !walls.contains(&moved.0) {
                if let std::collections::hash_map::Entry::Vacant(e) = shortest.entry(moved) {
                    e.insert((distance + 1, path2));
                } else {
                    match (distance + 1).cmp(&shortest[&moved].0) {
                        Ordering::Equal => {
                            for p in path2 {
                                if !shortest[&moved].1.contains(&p) {
                                    shortest.get_mut(&moved).unwrap().1.push(p);
                                }
                            }
                        }
                        Ordering::Less => {
                            *shortest.get_mut(&moved).unwrap() = (distance + 1, path2);
                        }
                        Ordering::Greater => {}
                    }
                }
            }
            for d in if pos.1 == (1, 0) || pos.1 == (-1, 0) {
                [(0, 1), (0, -1)]
            } else {
                [(1, 0), (-1, 0)]
            } {
                let value = shortest
                    .entry((pos.0, d))
                    .or_insert_with(|| (distance + 1000, path.clone()))
                    .0;
                if distance + 1000 < value {
                    *shortest.get_mut(&(pos.0, d)).unwrap() = (distance + 1000, path.clone());
                }
            }
        }

        if let Some(next_distance) = shortest
            .iter()
            .map(|(_, (i, _))| *i)
            .filter(|i| *i > distance)
            .min()
        {
            next = shortest
                .iter()
                .filter(|(_, (i, _))| *i == next_distance)
                .map(|(i, _)| *i)
                .collect::<Vec<_>>();
        } else {
            break;
        }
    }

    let s = min(
        min(shortest[&(end, (1, 0))].0, shortest[&(end, (-1, 0))].0),
        min(shortest[&(end, (0, 1))].0, shortest[&(end, (0, -1))].0),
    );
    let mut points = vec![];

    for (d, path) in [
        &shortest[&(end, (1, 0))],
        &shortest[&(end, (-1, 0))],
        &shortest[&(end, (0, 1))],
        &shortest[&(end, (0, -1))],
    ] {
        if *d == s {
            for p in path {
                if !points.contains(p) {
                    points.push(*p);
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(points.len(), 64);

    println!("{}", points.len());
}
