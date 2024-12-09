pub(crate) struct Grid {
    dimensions: (usize, usize),
    inner: Vec<Vec<char>>,
}

type Position = (isize, isize);

impl Grid {
    pub(crate) fn create(grid: Vec<Vec<char>>) -> Self {
        let dimensions = (grid.len(), grid[0].len());

        Self {
            dimensions,
            inner: grid,
        }
    }

    pub(crate) fn count_xmas(&self) -> usize {
        let mut num = 0;

        for (i, j) in self.positions('X') {
            let mut positions: Vec<(Position, Position)> = Vec::with_capacity(8);

            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    if self.validate_position((ni, nj)) {
                        positions.push(((ni, nj), (di, dj)));
                    }
                }
            }

            for position in positions {
                num += self.find_mas('M', position.0, position.1);
            }
        }

        num
    }

    fn find_mas(&self, current_char: char, position: Position, direction: Position) -> usize {
        if self.inner[position.0 as usize][position.1 as usize] != current_char {
            return 0;
        }

        match current_char {
            'M' => self.find_next('A', position, direction),
            'A' => self.find_next('S', position, direction),
            'S' => 1,
            _ => 0,
        }
    }

    fn find_next(&self, next_char: char, position: Position, direction: Position) -> usize {
        let next_position = (position.0 + direction.0, position.1 + direction.1);

        if self.validate_position(next_position) {
            self.find_mas(next_char, next_position, direction)
        } else {
            0
        }
    }

    fn positions(&self, c: char) -> Vec<(usize, usize)> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter().enumerate().filter_map(
                    move |(j, &char)| {
                        if char == c {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect()
    }

    fn validate_position(&self, position: Position) -> bool {
        position.0 >= 0
            && position.0 < self.dimensions.0 as isize
            && position.1 >= 0
            && position.1 < self.dimensions.1 as isize
    }
}
