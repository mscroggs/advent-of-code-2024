use std::cmp::min;
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
            let prize = (numbers[0], numbers[1]);
            machines.push(Machine { a, b, prize });
        } else if !line.is_empty() {
            panic!("Invalid line: {line}");
        }
    }

    let mut total = 0;

    for machine in &machines {
        let mut aim = (machine.prize.0, machine.prize.1);
        let mut presses = 500;
        for i in 1..=100 {
            aim.0 -= machine.a.0;
            aim.1 -= machine.a.1;
            if aim.0 % machine.b.0 == 0
                && aim.1 % machine.b.1 == 0
                && aim.0 / machine.b.0 == aim.1 / machine.b.1
                && aim.0 / machine.b.0 <= 100
            {
                presses = min(presses, i * 3 + aim.0 / machine.b.0);
            }
        }
        if presses != 500 {
            total += presses;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 480);

    println!("{total}");
}
