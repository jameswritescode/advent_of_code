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

    // In the grim darkness of the 21st century there is only this mess.
    pub(crate) fn find_word(&self, word: &str, directions: Vec<Position>) -> Vec<Vec<Position>> {
        let letters: Vec<char> = word.chars().collect();
        let word_len = letters.len();
        let mut word_positions: Vec<Vec<Position>> = vec![];

        for (i, line) in self.inner.iter().enumerate() {
            for (j, &current) in line.iter().enumerate() {
                if current != letters[0] {
                    continue;
                }

                let start_position = (i as isize, j as isize);

                for &(di, dj) in &directions {
                    let mut found_positions = vec![start_position];
                    let mut current_position = (start_position.0 + di, start_position.1 + dj);
                    let mut found_end = false;

                    for (li, &letter) in letters.iter().skip(1).enumerate() {
                        if !self.validate_position(current_position) {
                            break;
                        }

                        let lookup =
                            self.inner[current_position.0 as usize][current_position.1 as usize];

                        if lookup == letter {
                            found_positions.push(current_position);
                            current_position.0 += di;
                            current_position.1 += dj;
                        } else {
                            break;
                        }

                        if li == word_len - 2 {
                            found_end = true;
                            break;
                        }
                    }

                    if found_end {
                        word_positions.push(found_positions);
                    }
                }
            }
        }

        word_positions
    }

    fn validate_position(&self, position: Position) -> bool {
        position.0 >= 0
            && position.0 < self.dimensions.0 as isize
            && position.1 >= 0
            && position.1 < self.dimensions.1 as isize
    }
}
