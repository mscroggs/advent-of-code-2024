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

type CacheType = HashMap<((i64, i64), (i64, i64)), Vec<Vec<(i64, i64)>>>;
type FCacheType = HashMap<(Vec<(i64, i64)>, usize), usize>;
type AnotherCacheType = HashMap<((i64, i64), (i64, i64)), usize>;

trait Robot {
    type ButtonT: Copy;

    fn start() -> (i64, i64);
    fn get_coords(button: Self::ButtonT) -> (i64, i64);
    fn forbidden() -> Vec<(i64, i64)>;

    fn options(
        position: (i64, i64),
        target: (i64, i64),
        cache: &mut CacheType,
    ) -> Vec<Vec<(i64, i64)>> {
        if position == target {
            cache[&((0, 0), (0, 0))].clone()
        } else {
            if !cache.contains_key(&(position, target)) {
                let mut out = vec![];
                if position.1 > target.1
                    && !Self::forbidden().contains(&(position.0, position.1 - 1))
                {
                    for i in Self::options((position.0, position.1 - 1), target, cache) {
                        let mut v = vec![OtherRobot::get_coords(Button::Left)];
                        for j in i {
                            v.push(j);
                        }
                        out.push(v);
                    }
                }
                if position.1 < target.1
                    && !Self::forbidden().contains(&(position.0, position.1 + 1))
                {
                    for i in Self::options((position.0, position.1 + 1), target, cache) {
                        let mut v = vec![OtherRobot::get_coords(Button::Right)];
                        for j in i {
                            v.push(j);
                        }
                        out.push(v);
                    }
                }
                if position.0 > target.0
                    && !Self::forbidden().contains(&(position.0 - 1, position.1))
                {
                    for i in Self::options((position.0 - 1, position.1), target, cache) {
                        let mut v = vec![OtherRobot::get_coords(Button::Up)];
                        for j in i {
                            v.push(j);
                        }
                        out.push(v);
                    }
                }
                if position.0 < target.0
                    && !Self::forbidden().contains(&(position.0 + 1, position.1))
                {
                    for i in Self::options((position.0 + 1, position.1), target, cache) {
                        let mut v = vec![OtherRobot::get_coords(Button::Down)];
                        for j in i {
                            v.push(j);
                        }
                        out.push(v);
                    }
                }
                cache.insert((position, target), out);
            }
            cache[&(position, target)].clone()
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

fn shortest_other(
    line: &[(i64, i64)],
    remaining: usize,
    cache: &mut CacheType,
    f_cache: &mut FCacheType,
) -> usize {
    if remaining == 0 {
        line.len()
    } else {
        let key = (line.to_vec(), remaining);
        if !f_cache.contains_key(&key) {
            let value = line
                .iter()
                .enumerate()
                .map(|(n, c)| {
                    let prev = if n == 0 {
                        OtherRobot::start()
                    } else {
                        line[n - 1]
                    };
                    OtherRobot::options(prev, *c, cache)
                        .iter()
                        .map(|i| shortest_other(i, remaining - 1, cache, f_cache))
                        .min()
                        .unwrap()
                })
                .sum::<usize>();
            f_cache.insert(key.clone(), value);
        }
        f_cache[&key]
    }
}

fn shortest(
    line: &str,
    number: usize,
    cache: &mut AnotherCacheType,
    first_cache: &mut CacheType,
    other_cache: &mut CacheType,
    f_cache: &mut FCacheType,
) -> usize {
    let line = line.chars().collect::<Vec<_>>();
    line.iter()
        .enumerate()
        .map(|(n, c)| {
            let c = FirstRobot::get_coords(*c);
            let prev = if n == 0 {
                FirstRobot::start()
            } else {
                FirstRobot::get_coords(line[n - 1])
            };
            cache.entry((prev, c)).or_insert_with(|| {
                FirstRobot::options(prev, c, first_cache)
                    .iter()
                    .map(|i| shortest_other(i, number, other_cache, f_cache))
                    .min()
                    .unwrap()
            });
            cache[&(prev, c)]
        })
        .sum::<usize>()
}

fn main() {
    let mut f_cache = HashMap::new();

    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut cache = HashMap::new();
    let mut first_cache = HashMap::new();
    first_cache.insert(
        ((0, 0), (0, 0)),
        vec![vec![OtherRobot::get_coords(Button::A)]],
    );
    let mut other_cache = HashMap::new();
    other_cache.insert(
        ((0, 0), (0, 0)),
        vec![vec![OtherRobot::get_coords(Button::A)]],
    );

    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let number = line[..line.len() - 1].parse::<usize>().unwrap();
        #[cfg(not(feature = "test_input"))]
        let s = shortest(
            &line,
            25,
            &mut cache,
            &mut first_cache,
            &mut other_cache,
            &mut f_cache,
        );
        #[cfg(feature = "test_input")]
        let s = shortest(
            &line,
            2,
            &mut cache,
            &mut first_cache,
            &mut other_cache,
            &mut f_cache,
        );

        result += s * number;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 126384);

    println!("{result}");
}
