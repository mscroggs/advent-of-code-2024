use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut connections = vec![];
    let mut computers = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut cpus = vec![];
        for c in line.split("-") {
            if !computers.contains(&String::from(c)) {
                computers.push(String::from(c));
            }
            cpus.push(String::from(c));
        }
        assert_eq!(cpus.len(), 2);
        for c in [
            (cpus[0].clone(), cpus[1].clone()),
            (cpus[1].clone(), cpus[0].clone()),
        ] {
            if !connections.contains(&c) {
                connections.push(c);
            }
        }
    }

    let mut result = 0;
    for co in &connections {
        for c in &computers {
            if (co.0.starts_with("t") || co.1.starts_with("t") || c.starts_with("t"))
                && co.0 < co.1
                && co.1 < *c
                && connections.contains(&(co.0.clone(), c.clone()))
                && connections.contains(&(co.1.clone(), c.clone()))
            {
                result += 1;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 7);

    println!("{}", result);
}
