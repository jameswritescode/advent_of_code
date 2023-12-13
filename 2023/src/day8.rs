use std::{collections::BTreeMap, convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    network: Vec<(String, String)>,
    indexes: BTreeMap<String, usize>,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n");
        let instructions = parts.next().unwrap().chars().collect();
        let mut network = vec![];
        let mut indexes = BTreeMap::new();

        for (i, line) in parts.filter(|l| !l.is_empty()).enumerate() {
            let mut parts = line.split(" = ");
            let location = parts.next().unwrap().to_string();
            let coords = parts.next().unwrap();
            let coords_clipped: Vec<_> = coords[1..coords.len() - 1].split(", ").collect();
            let tuple = (coords_clipped[0].to_string(), coords_clipped[1].to_string());

            network.push(tuple);
            indexes.entry(location).or_insert(i);
        }

        Ok(Map {
            instructions,
            network,
            indexes,
        })
    }
}

fn solve_part1(map: &Map) -> u32 {
    let mut network_index = map.indexes["AAA"];
    let mut steps = 0;

    for &dir in map.instructions.iter().cycle() {
        let coords = &map.network[network_index];
        let next = if dir == 'R' { &coords.1 } else { &coords.0 };

        steps += 1;

        if next == "ZZZ" {
            break;
        } else {
            network_index = map.indexes[next];
        }
    }

    steps
}

pub fn run(input: String) {
    let map = input.parse::<Map>().unwrap();
    let part1_result = solve_part1(&map);

    println!("Part 1: {part1_result}");
}
