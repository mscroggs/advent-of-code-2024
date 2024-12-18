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

    #[cfg(feature = "test_input")]
    let size = 6;
    #[cfg(not(feature = "test_input"))]
    let size = 70;

    #[cfg(feature = "test_input")]
    let bytes = 12;
    #[cfg(not(feature = "test_input"))]
    let bytes = 1024;

    let coordinates = reader
        .lines()
        .map(|line| {
            let c = line
                .unwrap()
                .split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (c[0], c[1])
        })
        .collect::<Vec<_>>();

    let mut distances = HashMap::new();
    distances.insert((0, 0), 0);
    let mut new = vec![(0, 0)];

    loop {
        let distance = distances[&new[0]] + 1;

        for pos in new {
            for d in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let p = (pos.0 + d.0, pos.1 + d.1);
                if p.0 >= 0
                    && p.0 <= size
                    && p.1 >= 0
                    && p.1 <= size
                    && !coordinates[..bytes].contains(&p)
                {
                    let value = *distances.entry(p).or_insert_with(|| distance);
                    *distances.get_mut(&p).unwrap() = min(value, distance);
                }
            }
        }

        if distances.values().filter(|i| **i >= distance).count() == 0 {
            break;
        }

        let next = distances
            .values()
            .copied()
            .filter(|i| *i >= distance)
            .min()
            .unwrap();

        new = distances
            .iter()
            .filter(|(_, j)| **j == next)
            .map(|(i, _)| *i)
            .collect::<Vec<_>>();
    }

    let result = distances[&(size, size)];

    #[cfg(feature = "test_input")]
    assert_eq!(result, 22);

    println!("{result}");
}
