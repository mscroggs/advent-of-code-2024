use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Robot {
    pub p: (i64, i64),
    pub v: (i64, i64),
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    #[cfg(not(feature = "test_input"))]
    let shape = (101, 103);
    #[cfg(feature = "test_input")]
    let shape = (11, 7);

    let mut robots = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split(" ").collect::<Vec<_>>();
        let p = line[0].split("=").collect::<Vec<_>>()[1]
            .split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let v = line[1].split("=").collect::<Vec<_>>()[1]
            .split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let p = (p[0], p[1]);
        let v = (
            if v[0] < 0 { shape.0 + v[0] } else { v[0] },
            if v[1] < 0 { shape.1 + v[1] } else { v[1] },
        );
        robots.push(Robot { p, v });
    }

    // 10363
    for moves in 0..shape.0 * shape.1 {
        let counts = (0..shape.0)
            .map(|i| {
                (0..shape.1)
                    .map(|j| robots.iter().filter(|r| r.p == (i, j)).count())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if robots
            .iter()
            .enumerate()
            .map(|(i, r)| {
                robots
                    .iter()
                    .take(i)
                    .filter(|r2| {
                        (r2.p.0 == r.p.0 + 1 || r2.p.0 == r.p.0 - 1)
                            && (r2.p.1 == r.p.1 + 1 || r2.p.1 == r.p.1 - 1)
                    })
                    .count()
            })
            .sum::<usize>()
            > 300
        {
            println!("{moves}");
            for j in 0..shape.1 / 2 {
                for i in 0..shape.0 {
                    let c0 = counts[i as usize][2 * j as usize];
                    let c1 = counts[i as usize][2 * j as usize + 1];
                    if c0 > 0 {
                        if c1 > 0 {
                            print!("█");
                        } else {
                            print!("▀");
                        }
                    } else if c1 > 0 {
                        print!("▄");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!();
        }

        for robot in &mut robots {
            robot.p.0 += robot.v.0;
            robot.p.1 += robot.v.1;
            robot.p.0 %= shape.0;
            robot.p.1 %= shape.1;
        }
    }
}
