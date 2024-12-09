use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq)]
enum FileBlock {
    Empty,
    File(i64),
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().map(|i| i.unwrap()).collect::<Vec<_>>()[0].clone();

    let mut filesystem = vec![];
    let mut files = true;
    let mut file_id = 0;

    for c in line.chars() {
        let c = format!("{c}").parse::<i64>().unwrap();
        if files {
            for _ in 0..c {
                filesystem.push(FileBlock::File(file_id))
            }
            file_id += 1;
        } else {
            for _ in 0..c {
                filesystem.push(FileBlock::Empty)
            }
        }
        files = !files;
    }

    while filesystem.contains(&FileBlock::Empty) {
        if filesystem[filesystem.len() - 1] == FileBlock::Empty {
            while filesystem[filesystem.len() - 1] == FileBlock::Empty {
                filesystem.pop();
            }
        } else {
            let index = filesystem
                .iter()
                .enumerate()
                .filter(|(_, j)| **j == FileBlock::Empty)
                .map(|(i, _)| i)
                .collect::<Vec<_>>()[0];
            filesystem[index] = filesystem.pop().unwrap();
        }
    }
    for i in &filesystem {
        if let FileBlock::File(value) = *i {
            print!("{value}");
        } else {
            print!(".");
        }
    }
    println!();

    let mut total = 0;
    for (i, j) in filesystem.iter().enumerate() {
        if let FileBlock::File(value) = *j {
            total += i as i64 * value;
        } else {
            panic!();
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 1928);

    println!("{}", total);
}
