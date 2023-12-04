type GridVector = Vec<Vec<char>>;

struct Grid<'a> {
    rows: usize,
    cols: usize,
    vec: &'a GridVector,
}

fn valid_part_number(grid: &Grid, row: usize, col: usize) -> bool {
    let safe_up = row != 0;
    let safe_down = row != grid.rows - 1;
    let safe_left = col != 0;
    let safe_right = col != grid.cols - 1;

    let mut positions = vec![];

    if safe_left {
        positions.push((row, col - 1));
    }

    if safe_right {
        positions.push((row, col + 1));
    }

    if safe_down {
        positions.push((row + 1, col));

        if safe_right {
            positions.push((row + 1, col + 1));
        }
        if safe_left {
            positions.push((row + 1, col - 1));
        }
    }

    if safe_up {
        positions.push((row - 1, col));

        if safe_right {
            positions.push((row - 1, col + 1));
        }
        if safe_left {
            positions.push((row - 1, col - 1));
        }
    }

    positions.iter().any(|(r, c)| {
        let char = grid.vec[*r][*c];

        char != '.' && !char.is_ascii_digit()
    })
}

fn solve_part1(grid: &Grid) -> usize {
    let mut valid_parts: Vec<usize> = vec![];
    let mut valid_part_number_found = false;

    for (row, line) in grid.vec.iter().enumerate() {
        let mut part = String::new();

        for (col, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                part.push(*char);

                if valid_part_number(&grid, row, col) {
                    valid_part_number_found = true;
                }
            } else if !part.is_empty() {
                if valid_part_number_found {
                    valid_parts.push(part.parse().unwrap());

                    valid_part_number_found = false;
                }

                part.clear()
            }
        }

        if valid_part_number_found {
            valid_parts.push(part.parse().unwrap());
            part.clear();

            valid_part_number_found = false;
        }
    }

    valid_parts.iter().sum()
}

pub fn run(input: String) {
    let mut vector: GridVector = vec![];

    for line in input.lines() {
        vector.push(line.chars().collect());
    }

    let grid = Grid {
        rows: vector.len(),
        cols: vector[0].len(),
        vec: &vector,
    };

    let part1_result = solve_part1(&grid);

    println!("Part 1: {part1_result}");
}
