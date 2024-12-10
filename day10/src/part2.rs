use std::fs::File;
use std::io::{prelude::*, BufReader};

fn paths(map: &Vec<Vec<i32>>, start: (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    let value = map[start.0 as usize][start.1 as usize];
    if value == 9 {
        vec![vec![start]]
    } else {
        let mut out = vec![];
        for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let pos = (start.0 + direction.0, start.1 + direction.1);
            if 0 <= pos.0
                && pos.0 < map.len() as i32
                && 0 <= pos.1
                && pos.1 < map[0].len() as i32
                && map[pos.0 as usize][pos.1 as usize] == value + 1
            {
                for mut p in paths(map, pos) {
                    p.push(start);
                    out.push(p);
                }
            }
        }

        out
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let map = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|i| format!("{i}").parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if *entry == 0 {
                total += paths(&map, (i as i32, j as i32)).len();
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 81);

    println!("{}", total);
}
