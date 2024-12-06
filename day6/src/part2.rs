use std::fs::File;
use std::io::{prelude::*, BufReader};

fn loops(blocks: &[[i32; 2]], extra_block: [i32; 2], start: &[i32; 2], size: &(i32, i32)) -> bool {
    let mut dir = [-1, 0];
    let mut pos = [start[0], start[1]];

    let mut used = vec![];

    loop {
        while !blocks.contains(&pos) && pos != extra_block {
            if pos[0] < 0 || pos[0] >= size.0 || pos[1] < 0 || pos[1] >= size.1 {
                return false;
            }
            pos[0] += dir[0];
            pos[1] += dir[1];
        }
        pos[0] -= dir[0];
        pos[1] -= dir[1];
        dir = if dir == [0, 1] {
            [1, 0]
        } else if dir == [1, 0] {
            [0, -1]
        } else if dir == [0, -1] {
            [-1, 0]
        } else if dir == [-1, 0] {
            [0, 1]
        } else {
            panic!();
        };

        if used.contains(&[pos, dir]) {
            return true;
        }
        used.push([pos, dir]);
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut blocks = vec![];

    let mut start = [0, 0];

    let mut size = (0, 0);

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                blocks.push([row as i32, col as i32]);
            } else if c == '^' {
                start = [row as i32, col as i32];
            } else if c != '.' {
                panic!("Invalid character: {c}");
            }
            size.1 = col as i32 + 1;
        }
        size.0 = row as i32 + 1;
    }

    let mut positions = 0;

    for i in 0..size.0 {
        for j in 0..size.1 {
            if !blocks.contains(&[i, j]) && loops(&blocks, [i, j], &start, &size) {
                positions += 1;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(positions, 6);

    println!("{positions}");
}
