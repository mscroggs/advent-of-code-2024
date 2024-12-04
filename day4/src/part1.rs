use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.unwrap().chars().collect::<Vec<_>>());
    }

    let directions = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            for dir in &directions {
                let mut xmas = true;
                for (n, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
                    let i0 = i as i32 + dir.0 * n as i32;
                    let i1 = j as i32 + dir.1 * n as i32;
                    if i0 < 0
                        || i1 < 0
                        || lines.get(i0 as usize).is_none()
                        || Some(c) != lines[i0 as usize].get(i1 as usize)
                    {
                        xmas = false;
                        break;
                    }
                }
                if xmas {
                    count += 1;
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(count, 18);

    println!("{count}");
}
