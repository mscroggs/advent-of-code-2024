use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq)]
pub enum Op {
    XOR,
    AND,
    OR,
}

impl Op {
    fn from(s: &str) -> Option<Op> {
        match s {
            "AND" => Some(Op::AND),
            "OR" => Some(Op::OR),
            "XOR" => Some(Op::XOR),
            _ => None,
        }
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut first = true;
    let mut values = HashMap::new();
    let mut todo = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            first = false;
        } else if first {
            assert_eq!(line.chars().nth(3), Some(':'));
            assert_eq!(line.chars().nth(4), Some(' '));
            let vname = String::from(&line[..3]);
            let value = match line.chars().nth(5) {
                Some('1') => true,
                Some('0') => false,
                _ => panic!("invalid value: {line}"),
            };
            values.insert(vname, value);
        } else {
            let lsp = line.split(" ").collect::<Vec<_>>();
            todo.insert(
                String::from(lsp[4]),
                (
                    String::from(lsp[0]),
                    Op::from(lsp[1]).unwrap(),
                    String::from(lsp[2]),
                ),
            );
        }
    }

    while !todo.is_empty() {
        let mut done = vec![];
        for (i, j) in &todo {
            if values.contains_key(&j.0) && values.contains_key(&j.2) {
                values.insert(
                    i.clone(),
                    match j.1 {
                        Op::AND => values[&j.0] && values[&j.2],
                        Op::OR => values[&j.0] || values[&j.2],
                        Op::XOR => values[&j.0] ^ values[&j.2],
                    },
                );
                done.push(i.clone());
            }
        }
        for i in &done {
            todo.remove(i);
        }
    }

    let mut result = 0;
    for (i, j) in &values {
        if i.starts_with('z') && *j {
            result += i64::pow(2, i[1..].parse::<u32>().unwrap());
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 2024);

    println!("{}", result);
}
