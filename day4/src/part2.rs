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

    let mut count = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[0].len() - 1 {
            if lines[i][j] == 'A'
                && [('S', 'M'), ('M', 'S')].contains(&(lines[i + 1][j + 1], lines[i - 1][j - 1]))
                && [('S', 'M'), ('M', 'S')].contains(&(lines[i + 1][j - 1], lines[i - 1][j + 1]))
            {
                count += 1;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(count, 9);

    println!("{count}");
}
