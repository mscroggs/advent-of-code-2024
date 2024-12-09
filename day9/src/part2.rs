use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Block {
    pub id: i64,
    pub start: i64,
    pub len: i64,
}

#[derive(Debug)]
struct EmptyBlock {
    pub start: i64,
    pub len: i64,
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().map(|i| i.unwrap()).collect::<Vec<_>>()[0].clone();

    let mut files = true;
    let mut file_id = 0;
    let mut start = 0;

    let mut blocks = vec![];
    let mut empty = vec![];

    for c in line.chars() {
        let c = format!("{c}").parse::<i64>().unwrap();
        if files {
            blocks.push(Block {
                id: file_id,
                start,
                len: c,
            });
            file_id += 1;
        } else {
            empty.push(EmptyBlock { start, len: c });
        }
        start += c;
        files = !files;
    }

    for block in blocks.iter_mut().rev() {
        for e in empty.iter_mut() {
            if e.start > block.start {
                break;
            }
            if e.len >= block.len {
                block.start = e.start;
                e.start += block.len;
                e.len -= block.len;
                break;
            }
        }
    }

    let mut total = 0;

    for block in blocks.iter() {
        for i in block.start..block.start + block.len {
            total += i * block.id;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 2858);

    println!("{}", total);
}
