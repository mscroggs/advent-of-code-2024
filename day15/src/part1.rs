use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut blocks = vec![];
    let mut robots = vec![];
    let mut boxes = vec![];
    let mut directions = vec![];
    let mut firsthalf = true;
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            firsthalf = false;
        } else if firsthalf {
            for (col, c) in line.chars().enumerate() {
                let pos = (row as i64, col as i64);
                if c == '#' {
                    blocks.push(pos)
                } else if c == 'O' {
                    boxes.push(pos)
                } else if c == '@' {
                    robots.push(pos)
                } else {
                    assert_eq!(c, '.');
                }
            }
        } else {
            for i in line.chars() {
                directions.push(i);
            }
        }
    }

    for d in &directions {
        let m = match d {
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => {
                panic!();
            }
        };
        for r in &mut robots {
            let mut pos = (r.0 + m.0, r.1 + m.1);
            let mut move_boxes = vec![];
            while boxes.contains(&pos) {
                move_boxes.push({
                    let i = boxes
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| **x == pos)
                        .map(|(x, _)| x)
                        .collect::<Vec<_>>();
                    assert_eq!(i.len(), 1);
                    i[0]
                });
                pos.0 += m.0;
                pos.1 += m.1;
            }

            if !blocks.contains(&pos) {
                for i in move_boxes {
                    boxes[i].0 += m.0;
                    boxes[i].1 += m.1;
                }
                r.0 += m.0;
                r.1 += m.1;
            }
        }
    }

    let result = boxes.iter().map(|x| x.0 * 100 + x.1).sum::<i64>();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 10092);

    println!("{result}");
}
