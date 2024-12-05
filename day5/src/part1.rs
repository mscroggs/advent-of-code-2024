use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut rules = vec![];
    let mut preamble = true;

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            preamble = false;
        } else if preamble {
            let line = line.split("|").collect::<Vec<_>>();
            rules.push((
                line[0].parse::<i32>().unwrap(),
                line[1].parse::<i32>().unwrap(),
            ));
        } else {
            let book = line
                .split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut valid = true;
            for (i, a) in book.iter().enumerate() {
                for b in book.iter().take(i) {
                    if rules.contains(&(*a, *b)) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                total += book[book.len() / 2];
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 143);

    println!("{total}");
}
