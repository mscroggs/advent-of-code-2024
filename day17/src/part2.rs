use std::fs::File;
use std::io::{prelude::*, BufReader};

fn one_loop(a: i64, program: &[i64]) -> i64 {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;

    let mut ip = 0;
    loop {
        let combo = if [0, 2, 5, 6, 7].contains(&program[ip as usize]) {
            match program[ip as usize + 1] {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => a,
                5 => b,
                6 => c,
                7 => {
                    panic!("Cannot use reserved combo operand 7");
                }
                _ => {
                    panic!("Invalid combo operand");
                }
            }
        } else {
            0
        };
        let literal = program[ip as usize + 1];
        match program[ip as usize] {
            0 => {
                a /= i64::pow(2, combo as u32);
            }
            1 => {
                b ^= literal;
            }
            2 => {
                b = combo % 8;
            }
            3 => {
                panic!("This shouldn't happen");
            }
            4 => {
                b ^= c;
            }
            5 => {
                return combo % 8;
            }
            6 => {
                b = a / i64::pow(2, combo as u32);
            }
            7 => {
                c = a / i64::pow(2, combo as u32);
            }
            _ => {
                panic!("Invalid instruction");
            }
        }
        ip += 2;
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input_part2").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    assert!(lines[0].starts_with("Register A"));
    assert!(lines[1].starts_with("Register B: 0"));
    assert!(lines[2].starts_with("Register C: 0"));
    assert!(lines[3].is_empty());
    assert!(lines[4].starts_with("Program"));
    let program = lines[4].split("Program: ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut good = vec![0];
    for i in program.iter().rev() {
        let mut new_good = vec![];
        for g in good {
            for n in 8 * g..8 * g + 8 {
                if one_loop(n, &program) == *i {
                    new_good.push(n);
                }
            }
        }
        good = new_good;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(good[0], 117440);

    println!("{}", good[0]);
}
