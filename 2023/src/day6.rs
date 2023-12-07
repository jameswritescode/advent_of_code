type Parsed = Vec<Vec<usize>>;

fn calculate_possible_wins(time: usize, distance: usize) -> usize {
    let mut beat = 0;

    for i in 1..time {
        if (time - i) * i > distance {
            beat += 1;
        }
    }

    beat
}

fn solve_part1(parsed: &Parsed) -> usize {
    let series: Vec<_> = parsed[0].iter().zip(parsed[1].iter()).collect();

    series.iter().fold(1, |acc, (&time, &distance)| {
        acc * calculate_possible_wins(time, distance)
    })
}

fn solve_part2(parsed: &Parsed) -> usize {
    let numbers_str: Vec<String> = parsed
        .iter()
        .map(|v| v.iter().map(|n| n.to_string()).collect())
        .collect();

    let numbers: Vec<usize> = numbers_str.iter().map(|s| s.parse().unwrap()).collect();

    calculate_possible_wins(numbers[0], numbers[1])
}

pub fn run(input: String) {
    let parsed: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    let part1_result = solve_part1(&parsed);
    let part2_result = solve_part2(&parsed);

    println!("Part 1: {part1_result}");
    println!("Part 1: {part2_result}");
}
