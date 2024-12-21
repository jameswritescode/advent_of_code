use std::{collections::HashMap, env, fs};

type Position = (isize, isize);

struct Grid {
    dimensions: (usize, usize),
    inner: Vec<Vec<char>>,
    starting_position: Position,
}

impl Grid {
    fn new(inner: Vec<Vec<char>>) -> Self {
        let dimensions = (inner.len(), inner[0].len());
        let starting_position = Self::find_guard(&inner);

        Self {
            dimensions,
            inner,
            starting_position,
        }
    }

    fn find_guard(grid: &[Vec<char>]) -> Position {
        grid.iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(j, &char)| (char == '^').then_some((i as isize, j as isize)))
            })
            .next()
            .unwrap()
    }

    fn validate_position(&self, position: Position) -> bool {
        position.0 >= 0
            && position.0 < self.dimensions.0 as isize
            && position.1 >= 0
            && position.1 < self.dimensions.1 as isize
    }

    fn guard_positions(&self) -> HashMap<Position, usize> {
        let mut directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut guard_positions = HashMap::new();
        let mut guard_position = self.starting_position;

        loop {
            *guard_positions.entry(guard_position).or_insert(0) += 1;

            let next_position = (
                guard_position.0 + directions[0].0,
                guard_position.1 + directions[0].1,
            );

            if !self.validate_position(next_position) {
                break;
            }

            if self.inner[next_position.0 as usize][next_position.1 as usize] == '#' {
                directions.rotate_left(1);
            } else {
                guard_position = next_position;
            }
        }

        guard_positions
    }
}

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let grid = Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    println!("{}", grid.guard_positions().len());
}
