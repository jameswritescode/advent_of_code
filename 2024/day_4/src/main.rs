mod grid;

use std::{env, fs};

use grid::Grid;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let grid = Grid::create(input.lines().map(|l| l.chars().collect()).collect());

    part1(&grid);
}

fn part1(grid: &Grid) {
    println!("{}", grid.count_xmas());
}
