use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    #[cfg(not(feature = "test_input"))]
    let shape = (101, 103);
    #[cfg(feature = "test_input")]
    let shape = (11, 7);

    let mut counts = [0, 0, 0, 0];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split(" ").collect::<Vec<_>>();
        let mut p = line[0].split("=").collect::<Vec<_>>()[1]
            .split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let v = {
            let v = line[1].split("=").collect::<Vec<_>>()[1]
                .split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            vec![
                if v[0] < 0 { shape.0 + v[0] } else { v[0] },
                if v[1] < 0 { shape.1 + v[1] } else { v[1] },
            ]
        };

        p[0] += 100 * v[0];
        p[1] += 100 * v[1];
        p[0] %= shape.0;
        p[1] %= shape.1;
        if p[0] != shape.0 / 2 && p[1] != shape.1 / 2 {
            if p[0] < shape.0 / 2 {
                if p[1] < shape.1 / 2 {
                    counts[0] += 1;
                } else {
                    counts[1] += 1;
                }
            } else if p[1] < shape.1 / 2 {
                counts[2] += 1;
            } else {
                counts[3] += 1;
            }
        }
    }

    let result = counts[0] * counts[1] * counts[2] * counts[3];

    #[cfg(feature = "test_input")]
    assert_eq!(result, 12);

    println!("{result}");
}
