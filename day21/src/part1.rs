use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
}

type CacheType = HashMap<((i64, i64), (i64, i64)), Vec<Vec<Button>>>;

fn options<'a>(
    position: (i64, i64),
    target: (i64, i64),
    cache: &'a mut CacheType,
    forbidden: &[(i64, i64)],
) -> &'a [Vec<Button>] {
    if position == target {
        &cache[&((0, 0), (0, 0))]
    } else {
        if !cache.contains_key(&(position, target)) {
            let mut out = vec![];
            if position.1 > target.1 && !forbidden.contains(&(position.0, position.1 - 1)) {
                for i in options((position.0, position.1 - 1), target, cache, forbidden) {
                    let mut v = vec![Button::Left];
                    for j in i {
                        v.push(*j);
                    }
                    out.push(v);
                }
            }
            if position.1 < target.1 && !forbidden.contains(&(position.0, position.1 + 1)) {
                for i in options((position.0, position.1 + 1), target, cache, forbidden) {
                    let mut v = vec![Button::Right];
                    for j in i {
                        v.push(*j);
                    }
                    out.push(v);
                }
            }
            if position.0 > target.0 && !forbidden.contains(&(position.0 - 1, position.1)) {
                for i in options((position.0 - 1, position.1), target, cache, forbidden) {
                    let mut v = vec![Button::Up];
                    for j in i {
                        v.push(*j);
                    }
                    out.push(v);
                }
            }
            if position.0 < target.0 && !forbidden.contains(&(position.0 + 1, position.1)) {
                for i in options((position.0 + 1, position.1), target, cache, forbidden) {
                    let mut v = vec![Button::Down];
                    for j in i {
                        v.push(*j);
                    }
                    out.push(v);
                }
            }
            cache.insert((position, target), out);
        }
        &cache[&(position, target)]
    }
}

trait Robot {
    type ButtonT: Copy;

    fn start() -> (i64, i64);
    fn get_coords(button: Self::ButtonT) -> (i64, i64);
    fn forbidden() -> Vec<(i64, i64)>;

    fn get_all(
        start: (i64, i64),
        buttons: &[Self::ButtonT],
        cache: &mut CacheType,
    ) -> Vec<Vec<Button>> {
        if buttons.is_empty() {
            vec![vec![]]
        } else {
            let mut out = vec![];
            let target = Self::get_coords(buttons[0]);
            let next = Self::get_all(target, &buttons[1..], cache);
            for seq in options(start, target, cache, &Self::forbidden()) {
                for n in &next {
                    let mut s = vec![];
                    for i in seq {
                        s.push(*i);
                    }
                    for i in n {
                        s.push(*i);
                    }
                    out.push(s);
                }
            }
            out
        }
    }
}

struct FirstRobot {}

impl Robot for FirstRobot {
    type ButtonT = char;

    fn start() -> (i64, i64) {
        (3, 2)
    }

    fn get_coords(button: char) -> (i64, i64) {
        match button {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => {
                panic!("Unsupported button: {button}");
            }
        }
    }

    fn forbidden() -> Vec<(i64, i64)> {
        vec![(3, 0)]
    }
}

struct OtherRobot {}

impl Robot for OtherRobot {
    type ButtonT = Button;

    fn start() -> (i64, i64) {
        (0, 2)
    }

    fn get_coords(button: Button) -> (i64, i64) {
        match button {
            Button::Up => (0, 1),
            Button::A => (0, 2),
            Button::Left => (1, 0),
            Button::Down => (1, 1),
            Button::Right => (1, 2),
        }
    }

    fn forbidden() -> Vec<(i64, i64)> {
        vec![(0, 0)]
    }
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut cache = HashMap::new();
    cache.insert(((0, 0), (0, 0)), vec![vec![Button::A]]);
    let mut second_cache = HashMap::new();
    second_cache.insert(((0, 0), (0, 0)), vec![vec![Button::A]]);

    let cache_ref = &mut cache;
    let second_cache_ref = &mut second_cache;

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut shortest = vec![];
        for i in &FirstRobot::get_all(
            FirstRobot::start(),
            &line.chars().collect::<Vec<_>>(),
            cache_ref,
        ) {
            for j in &OtherRobot::get_all(OtherRobot::start(), i, second_cache_ref) {
                for k in OtherRobot::get_all(OtherRobot::start(), j, second_cache_ref) {
                    if shortest.is_empty() || k.len() < shortest.len() {
                        shortest = k;
                    }
                }
            }
        }

        result += shortest.len() * line[..line.len() - 1].parse::<usize>().unwrap();
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 126384);

    println!("{result}");
}
