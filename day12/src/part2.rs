use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let map = reader
        .lines()
        .map(|i| i.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut used = vec![];

    let mut total = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            let coord = (i as i32, j as i32);
            if !used.contains(&coord) {
                let mut area = vec![coord];
                let mut new = vec![coord];
                used.push(coord);
                while !new.is_empty() {
                    let mut newnew = vec![];
                    for pos in &new {
                        for add in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                            let next = (pos.0 + add.0, pos.1 + add.1);
                            if !area.contains(&next)
                                && next.0 >= 0
                                && next.0 < map.len() as i32
                                && next.1 >= 0
                                && next.1 < map[0].len() as i32
                                && map[next.0 as usize][next.1 as usize] == *entry
                            {
                                newnew.push(next);
                                area.push(next);
                                used.push(next);
                            }
                        }
                    }
                    new = newnew;
                }
                let mut sides = 0;
                for pos in &area {
                    if !area.contains(&(pos.0 + 1, pos.1))
                        && (area.contains(&(pos.0 + 1, pos.1 + 1))
                            || !area.contains(&(pos.0, pos.1 + 1)))
                    {
                        sides += 1;
                    }
                    if !area.contains(&(pos.0 - 1, pos.1))
                        && (area.contains(&(pos.0 - 1, pos.1 + 1))
                            || !area.contains(&(pos.0, pos.1 + 1)))
                    {
                        sides += 1;
                    }
                    if !area.contains(&(pos.0, pos.1 + 1))
                        && (area.contains(&(pos.0 + 1, pos.1 + 1))
                            || !area.contains(&(pos.0 + 1, pos.1)))
                    {
                        sides += 1;
                    }
                    if !area.contains(&(pos.0, pos.1 - 1))
                        && (area.contains(&(pos.0 + 1, pos.1 - 1))
                            || !area.contains(&(pos.0 + 1, pos.1)))
                    {
                        sides += 1;
                    }
                }
                total += area.len() * sides;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(total, 1206);

    println!("{total}");
}
