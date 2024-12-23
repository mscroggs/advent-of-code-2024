use std::fs::File;
use std::io::{prelude::*, BufReader};

fn find_lan_party(
    done: &[Vec<String>],
    computers: &[String],
    connections: &[(String, String)],
) -> Vec<Vec<String>> {
    let mut longer = vec![];
    for d in done {
        for c in computers {
            if d.is_empty() || *c > d[d.len() - 1] {
                let mut connected = true;
                for c2 in d {
                    if !connections.contains(&(c2.clone(), c.clone())) {
                        connected = false;
                        break;
                    }
                }
                if connected {
                    let mut g = d.clone();
                    g.push(c.clone());
                    longer.push(g);
                }
            }
        }
    }
    if longer.is_empty() {
        done.to_vec()
    } else {
        find_lan_party(&longer, computers, connections)
    }
}

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
            if c.0 < c.1 && !connections.contains(&c) {
                connections.push(c);
            }
        }
    }

    computers.sort();

    let party = find_lan_party(&[vec![]], &computers, &connections);
    let result = party[0].join(",");

    #[cfg(feature = "test_input")]
    assert_eq!(result, "co,de,ka,ta");

    println!("{}", result);
}
