use std::cmp::max;
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

fn compute(
    input_values: &HashMap<String, bool>,
    todo: &HashMap<String, (String, Op, String)>,
    swaps: &HashMap<String, String>,
) -> i64 {
    let mut values = HashMap::new();
    for (i, j) in input_values {
        values.insert(i.clone(), *j);
    }
    let mut changed = true;
    while changed {
        changed = false;
        for (i, j) in todo {
            let i = if swaps.contains_key(i) {
                swaps[i].clone()
            } else {
                i.clone()
            };
            if !values.contains_key(&i) && values.contains_key(&j.0) && values.contains_key(&j.2) {
                values.insert(
                    i,
                    match j.1 {
                        Op::AND => values[&j.0] && values[&j.2],
                        Op::OR => values[&j.0] || values[&j.2],
                        Op::XOR => values[&j.0] ^ values[&j.2],
                    },
                );
                changed = true;
            }
        }
    }

    let mut result = 0;
    for (i, j) in &values {
        if i.starts_with('z') && *j {
            result += i64::pow(2, i[1..].parse::<u32>().unwrap());
        }
    }
    result
}

#[cfg(feature = "test_input")]
fn add(values: &HashMap<String, bool>) -> i64 {
    let mut result = 0;
    for (i, j) in values {
        if i.starts_with('x') && *j && values[&format!("y{}", &i[1..])] {
            result += i64::pow(2, i[1..].parse::<u32>().unwrap());
        }
    }
    result
}

#[cfg(not(feature = "test_input"))]
fn add(values: &HashMap<String, bool>) -> i64 {
    let mut result = 0;
    for (i, j) in values {
        if i.starts_with('x') && *j {
            result += i64::pow(2, i[1..].parse::<u32>().unwrap());
        }
        if i.starts_with('y') && *j {
            result += i64::pow(2, i[1..].parse::<u32>().unwrap());
        }
    }
    result
}

fn find_options(
    keys: &[String],
    values: &HashMap<String, bool>,
    todo: &HashMap<String, (String, Op, String)>,
    npairs: usize,
    target: i64,
    pairs: &[(usize, usize)],
) -> Vec<HashMap<String, String>> {
    if pairs.len() == npairs {
        let mut swaps = HashMap::new();
        for i in pairs {
            swaps.insert(keys[i.0].clone(), keys[i.1].clone());
            swaps.insert(keys[i.1].clone(), keys[i.0].clone());
        }
        if compute(values, todo, &swaps) == target {
            vec![swaps]
        } else {
            vec![]
        }
    } else {
        let mut options = vec![];
        for i in if pairs.is_empty() { 0 } else { pairs[0].0 }..keys.len() {
            for j in i + 1..keys.len() {
                let mut ok = true;
                for p in pairs {
                    if i == p.0 || j == p.0 || i == p.1 || j == p.1 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    let mut new_pairs = vec![];
                    for p in pairs {
                        new_pairs.push((p.0, p.1));
                    }
                    new_pairs.push((i, j));
                    for o in find_options(keys, values, todo, npairs, target, &new_pairs) {
                        options.push(o);
                    }
                }
            }
        }
        options
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input_part2").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut first = true;
    let mut values = HashMap::new();
    let mut todo = HashMap::new();
    let mut digits = 0;

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
            digits = max(digits, line[1..3].parse::<u32>().unwrap());
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

    #[cfg(feature = "test_input")]
    let mut options = find_options(
        &todo.keys().cloned().collect::<Vec<_>>(),
        &values,
        &todo,
        2,
        add(&values),
        &[],
    );
    #[cfg(not(feature = "test_input"))]
    let mut options = find_options(
        &todo.keys().cloned().collect::<Vec<_>>(),
        &values,
        &todo,
        3,
        add(&values),
        &[],
    );

    let mut x = 0;
    let mut y = 0;
    while options.len() > 1 {
        let mut values = HashMap::new();

        let mut fails = vec![];

        for (index, swaps) in options.iter().enumerate() {
            for i in 0..=digits {
                values.insert(
                    if i < 10 {
                        format!("x0{i}")
                    } else {
                        format!("x{i}")
                    },
                    (x >> i) & 1 == 1,
                );
                values.insert(
                    if i < 10 {
                        format!("y0{i}")
                    } else {
                        format!("y{i}")
                    },
                    (y >> i) & 1 == 1,
                );
            }
            if compute(&values, &todo, swaps) != add(&values) {
                fails.push(index);
            }
        }
        for i in fails.iter().rev() {
            options.remove(*i);
        }
        x += 1;
        if x == i64::pow(2, digits + 1) {
            x = 0;
            y += 1;
        }
        if y == i64::pow(2, digits + 1) {
            panic!();
        }
    }

    let mut result = options[0].keys().cloned().collect::<Vec<_>>();
    result.sort();
    let result = result.join(",");

    #[cfg(feature = "test_input")]
    assert_eq!(result, "z00,z01,z02,z05");

    println!("{}", result);
}
