fn solve_part1(line: &str) -> u32 {
    let data: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    if let Some(first) = data.first() {
        10 * first + data.last().unwrap()
    } else {
        0
    }
}

// Hinted at but not shown in action by the example of part 2 is that words can overlap e.g.
// eighthree should become 83 and not 8hree.
const REPLACEMENTS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn solve_part2(line: &str) -> u32 {
    let mut new_line = String::new();

    line.chars().for_each(|c| {
        new_line.push(c);

        for (word, num) in REPLACEMENTS {
            new_line = new_line.replace(word, num);
        }
    });

    solve_part1(new_line.as_str())
}

pub fn run(input: String) {
    let mut part1_result = 0;
    let mut part2_result = 0;

    for line in input.lines() {
        part1_result += solve_part1(line);
        part2_result += solve_part2(line);
    }

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
