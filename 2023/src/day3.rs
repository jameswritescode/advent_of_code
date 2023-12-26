// h/t https://is.gd/rSrqFZ

use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Part {
    number: u32,
    position: usize,
}

impl Part {
    fn positions(&self, dimension: usize) -> Vec<usize> {
        let west = self.position.saturating_sub(1);
        let east = self.position + self.number.to_string().len();

        let mut positions = vec![west, east];

        for n in west..=east {
            let up = n.saturating_sub(dimension);

            if up != 0 {
                positions.push(up);
            }

            let down = n + dimension;

            if down < dimension * dimension {
                positions.push(down);
            }
        }

        positions
    }
}

#[derive(Debug)]
struct Schematic {
    dimension: usize,
    parts: Vec<Part>,
    symbols: Vec<char>,
}

impl FromStr for Schematic {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dimension = s.lines().count();
        let mut parts = vec![];
        let mut symbols = vec!['.'; dimension * dimension];

        for (i, row) in s.lines().enumerate() {
            let mut number_position: Option<usize> = None;
            let mut number = 0;

            for (j, col) in row.chars().enumerate() {
                let current_position = j + row.len() * i;

                if col.is_ascii_digit() {
                    let num = col.to_digit(10).unwrap();

                    if number_position.is_some() {
                        number = number * 10 + num;
                    } else {
                        number_position = Some(current_position);
                        number = num;
                    }
                } else {
                    if let Some(n) = number_position {
                        parts.push(Part {
                            position: n,
                            number,
                        });

                        number = 0;
                        number_position = None;
                    }

                    symbols[current_position] = col;
                }
            }

            if let Some(n) = number_position {
                parts.push(Part {
                    position: n,
                    number,
                });
            }
        }

        Ok(Schematic {
            parts,
            symbols,
            dimension,
        })
    }
}

fn solve_part1(schematic: &Schematic) -> u32 {
    schematic
        .parts
        .iter()
        .filter_map(|p| {
            if p.positions(schematic.dimension)
                .iter()
                .any(|&p| schematic.symbols[p] != '.')
            {
                Some(p.number)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(schematic: &Schematic) -> u32 {
    let mut ratios = vec![vec![]; schematic.symbols.len()];

    for part in &schematic.parts {
        for pos in part.positions(schematic.dimension) {
            if schematic.symbols[pos] == '*' {
                ratios[pos].push(part.number);
            }
        }
    }

    ratios
        .iter()
        .map(|pn| {
            if pn.len() > 1 {
                pn.iter().fold(1, |acc, n| acc * n)
            } else {
                0
            }
        })
        .sum()
}

pub fn run(input: String) {
    let schematic = input.parse::<Schematic>().unwrap();
    let part1_result = solve_part1(&schematic);
    let part2_result = solve_part2(&schematic);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
