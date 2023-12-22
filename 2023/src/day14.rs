type Grid = Vec<Vec<char>>;

fn solve_part1(grid: &mut Grid) -> usize {
    let grid_len = grid.len();
    let mut rocks: Vec<i32> = vec![-1; grid_len];

    for i in 0..grid_len {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '#' => rocks[j] = i as i32,
                'O' => {
                    let mut index = i;
                    let next_placement = (rocks[j] + 1) as usize;

                    if next_placement != i {
                        index = next_placement;
                        grid[index][j] = 'O';
                        grid[i][j] = '.';
                    }

                    rocks[j] = index as i32;
                }
                _ => {}
            }
        }
    }

    grid.iter().enumerate().fold(0, |acc, (i, s)| {
        let rocks = s.iter().filter(|&c| c == &'O').count();

        acc + (rocks * (grid_len - i))
    })
}

pub fn run(input: String) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let part1_result = solve_part1(&mut grid.clone());

    println!("Part 1: {part1_result}");
}
