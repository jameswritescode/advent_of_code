mod grid;

use std::{collections::HashMap, env, fs};

use grid::Grid;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let grid = Grid::create(input.lines().map(|l| l.chars().collect()).collect());

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let num = grid
        .find_word(
            "XMAS",
            vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
        )
        .len();

    println!("{num}");
}

fn part2(grid: &Grid) {
    let num = grid
        .find_word("MAS", vec![(-1, -1), (-1, 1), (1, -1), (1, 1)])
        .iter()
        .map(|w| w[1])
        .fold(HashMap::new(), |mut acc, pos| {
            *acc.entry(pos).or_insert(0) += 1;
            acc
        })
        .values()
        .filter(|&&n| n > 1)
        .count();

    println!("{num}");
}
