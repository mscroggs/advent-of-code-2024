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
                let pos0 = (row as i64, 2 * col as i64);
                let pos1 = (row as i64, 2 * col as i64 + 1);
                if c == '#' {
                    blocks.push(pos0);
                    blocks.push(pos1);
                } else if c == 'O' {
                    boxes.push(pos0);
                } else if c == '@' {
                    robots.push(pos0);
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
            let mut move_boxes = vec![];
            let mut pos = vec![(r.0 + m.0, r.1 + m.1)];
            let mut found_box = false;
            while !pos.is_empty() {
                for p in &pos {
                    if blocks.contains(p) {
                        found_box = true;
                        break;
                    }
                }
                if found_box {
                    break;
                }

                let boxes_to_add = boxes
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| pos.contains(x) || pos.contains(&(x.0, x.1 + 1)))
                    .map(|(x, _)| x)
                    .filter(|x| !move_boxes.contains(x))
                    .collect::<Vec<_>>();

                for i in &boxes_to_add {
                    move_boxes.push(*i);
                }
                pos = vec![];
                for p in &boxes_to_add {
                    pos.push((boxes[*p].0 + m.0, boxes[*p].1 + m.1));
                    pos.push((boxes[*p].0 + m.0, boxes[*p].1 + m.1 + 1));
                }
            }

            if !found_box {
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
    assert_eq!(result, 9021);

    println!("{result}");
}
