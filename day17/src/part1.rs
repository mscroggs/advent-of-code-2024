use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    assert!(lines[0].starts_with("Register A"));
    let mut a = lines[0].split("Register A: ").collect::<Vec<_>>()[1]
        .parse::<i64>()
        .unwrap();
    assert!(lines[1].starts_with("Register B"));
    let mut b = lines[1].split("Register B: ").collect::<Vec<_>>()[1]
        .parse::<i64>()
        .unwrap();
    assert!(lines[2].starts_with("Register C"));
    let mut c = lines[2].split("Register C: ").collect::<Vec<_>>()[1]
        .parse::<i64>()
        .unwrap();
    assert!(lines[3].is_empty());
    assert!(lines[4].starts_with("Program"));

    let program = lines[4].split("Program: ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut ip = 0;
    let mut output = vec![];
    while (ip as usize) < program.len() {
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
        let literal = program[ip as usize + 1] as i64;
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
                if a != 0 {
                    ip = literal - 2;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                output.push(combo % 8);
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
        ip += 2
    }

    let result = output
        .iter()
        .map(|i| format!("{i}"))
        .collect::<Vec<_>>()
        .join(",");

    #[cfg(feature = "test_input")]
    assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");

    println!("{result}");
}
