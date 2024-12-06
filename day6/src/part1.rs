use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut blocks = vec![];

    let mut pos = [[0, 0], [-1, -1]];

    let mut size = (0, 0);

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                blocks.push([row as i32, col as i32]);
            } else if c == '^' {
                pos[0] = [row as i32, col as i32];
                pos[1] = [-1, 0];
            } else if c != '.' {
                panic!("Invalid character: {c}");
            }
            size.1 = col as i32 + 1;
        }
        size.0 = row as i32 + 1;
    }

    let mut visited = vec![pos[0]];

    loop {
        while !blocks.contains(&pos[0]) {
            if pos[0][0] < 0 || pos[0][0] >= size.0 || pos[0][1] < 0 || pos[0][1] >= size.1 {
                break;
            }
            if !visited.contains(&pos[0]) {
                visited.push(pos[0]);
            }
            pos[0][0] += pos[1][0];
            pos[0][1] += pos[1][1];
        }
        if pos[0][0] < 0 || pos[0][0] >= size.0 || pos[0][1] < 0 || pos[0][1] >= size.1 {
            break;
        }
        pos[0][0] -= pos[1][0];
        pos[0][1] -= pos[1][1];
        pos[1] = if pos[1] == [0, 1] {
            [1, 0]
        } else if pos[1] == [1, 0] {
            [0, -1]
        } else if pos[1] == [0, -1] {
            [-1, 0]
        } else if pos[1] == [-1, 0] {
            [0, 1]
        } else {
            panic!();
        };
    }

    #[cfg(feature = "test_input")]
    assert_eq!(visited.len(), 41);

    println!("{}", visited.len());
}
