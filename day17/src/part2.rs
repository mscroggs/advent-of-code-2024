use std::fs::File;
use std::io::{prelude::*, BufReader};

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

    let mut start = 0;
    loop {
        let mut a = start;
        let mut b = 0;
        let mut c = 0;

        let mut ip = 0;
        let mut output = vec![];
        while ip < program.len() {
            let combo = if [0, 2, 5, 6, 7].contains(&program[ip]) {
                match program[ip + 1] {
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
            let literal = program[ip + 1];
            match program[ip] {
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
                        ip = literal as usize - 2;
                    }
                }
                4 => {
                    b ^= c;
                }
                5 => {
                    output.push(combo % 8);
                    if output.len() > program.len()
                        || output[output.len() - 1] != program[output.len() - 1]
                    {
                        break;
                    }
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
        if output.len() > 9 {
            println!("{start} {output:?}");
        }
        if output == program {
            break;
        }
        start += 1;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(start, 117440);

    println!("{start}");
}
