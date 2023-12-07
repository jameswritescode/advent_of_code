use std::{convert::Infallible, str::FromStr};

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<Vec<usize>>>,
}

impl FromStr for Almanac {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n");

        let seeds = parts
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        let mut maps = vec![];
        let mut current_map = 0;

        for line in parts.filter(|&c| c != "") {
            if line.contains("map") {
                current_map += 1;
                maps.push(vec![]);
                continue;
            }

            let map: Vec<usize> = line
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();

            maps[current_map - 1].push(map);
        }

        Ok(Self { seeds, maps })
    }
}

fn lowest_location_from_seeds(almanac: &Almanac, seeds: &Vec<usize>) -> usize {
    seeds
        .iter()
        .map(|&s| {
            let mut destination = s;

            for maps in almanac.maps.iter() {
                for map in maps {
                    let source_range = map[1]..map[1] + map[2];

                    if source_range.contains(&destination) {
                        destination = map[0] + destination - map[1];
                        break;
                    }
                }
            }

            destination
        })
        .min()
        .unwrap()
}

fn solve_part1(almanac: &Almanac) -> usize {
    lowest_location_from_seeds(&almanac, &almanac.seeds)
}

// This is INCREDIBLY slow and should be smarter about choosing seeds.
fn solve_part2(almanac: &Almanac) -> usize {
    let seeds: Vec<_> = almanac
        .seeds
        .chunks(2)
        .flat_map(|c| c[0]..c[0] + c[1])
        .collect();

    lowest_location_from_seeds(&almanac, &seeds)
}

pub fn run(input: String) {
    let almanac = input.parse::<Almanac>().unwrap();
    let part1_result = solve_part1(&almanac);
    let part2_result = solve_part2(&almanac);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
