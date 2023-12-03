use std::collections::HashMap;

struct Game<'a> {
    id: u32,
    cubes: HashMap<&'a str, u32>,
}

fn solve_part1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|game| {
            let cubes = &game.cubes;

            cubes.get("red").unwrap() <= &12
                && cubes.get("green").unwrap() <= &13
                && cubes.get("blue").unwrap() <= &14
        })
        .map(|g| g.id)
        .sum()
}

fn solve_part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| game.cubes.iter().fold(1, |acc, (_color, &num)| acc * num))
        .sum()
}

fn parse_games<'a>(input: &'a String) -> Vec<Game<'a>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let mut cubes = HashMap::new();

            parts[1].split("; ").for_each(|sets| {
                let set_list = sets.split(", ");

                set_list.for_each(|set| {
                    let draw: Vec<&str> = set.split(" ").collect();
                    let num = draw[0].parse::<u32>().unwrap();

                    cubes
                        .entry(draw[1])
                        .and_modify(|n| {
                            if *n < num {
                                *n = num
                            }
                        })
                        .or_insert(num);
                })
            });

            Game {
                id: parts[0].split(" ").last().unwrap().parse::<u32>().unwrap(),
                cubes,
            }
        })
        .collect()
}

pub fn run(input: String) {
    let games = parse_games(&input);
    let part1_result = solve_part1(&games);
    let part2_result = solve_part2(&games);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
