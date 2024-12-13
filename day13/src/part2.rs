use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Machine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64),
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut machines = vec![];

    let mut a = (0, 0);
    let mut b = (0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("Button A: ") {
            let numbers = line
                .split("+")
                .skip(1)
                .map(|i| i.split(",").collect::<Vec<_>>()[0].parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            a = (numbers[0], numbers[1]);
        } else if line.starts_with("Button B: ") {
            let numbers = line
                .split("+")
                .skip(1)
                .map(|i| i.split(",").collect::<Vec<_>>()[0].parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            b = (numbers[0], numbers[1]);
        } else if line.starts_with("Prize: ") {
            let numbers = line
                .split("=")
                .skip(1)
                .map(|i| i.split(",").collect::<Vec<_>>()[0].parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            #[cfg(feature = "test_input")]
            let prize = (numbers[0], numbers[1]);
            #[cfg(not(feature = "test_input"))]
            let prize = (numbers[0] + 10000000000000, numbers[1] + 10000000000000);
            machines.push(Machine { a, b, prize });
        } else if !line.is_empty() {
            panic!("Invalid line: {line}");
        }
    }

    let mut total = 0;

    for machine in &machines {
        let (det, i, j) = {
            let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
            let i = machine.b.1 * machine.prize.0 - machine.b.0 * machine.prize.1;
            let j = machine.a.0 * machine.prize.1 - machine.a.1 * machine.prize.0;
            if det > 0 {
                (det, i, j)
            } else {
                (-det, -i, -j)
            }
        };
        if det >= 0 && i >= 0 && j >= 0 && i % det == 0 && j % det == 0 {
            total += (3 * i + j) / det;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 480);

    println!("{total}");
}
