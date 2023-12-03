use std::cmp::max;

struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn solve_part1(games: &Vec<Cubes>) -> usize {
    games
        .iter()
        .enumerate()
        .filter(|(_, cubes)| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn solve_part2(games: &Vec<Cubes>) -> usize {
    games
        .iter()
        .map(|cubes| cubes.red * cubes.blue * cubes.green)
        .sum()
}

fn parse_cubes(input: String) -> Vec<Cubes> {
    input
        .lines()
        .map(|line| {
            let mut cubes = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };

            line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .for_each(|sets| {
                    for set in sets.split(", ") {
                        let mut parts = set.split(" ");
                        let num = parts.next().unwrap().parse().unwrap();
                        let color = parts.next().unwrap();

                        match color {
                            "red" => cubes.red = max(num, cubes.red),
                            "green" => cubes.green = max(num, cubes.green),
                            "blue" => cubes.blue = max(num, cubes.blue),
                            _ => panic!("Unknown color: {color}"),
                        }
                    }
                });

            cubes
        })
        .collect()
}

pub fn run(input: String) {
    let cubes = parse_cubes(input);
    let part1_result = solve_part1(&cubes);
    let part2_result = solve_part2(&cubes);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
