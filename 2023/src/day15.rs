fn hash(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, n| ((acc + n) * 17) % 256)
}

fn find_label(s: &str) -> &str {
    &s.split_terminator(&['=', '-'][..]).nth(0).unwrap()
}

fn solve_part1(seq: &Vec<&str>) -> u32 {
    seq.iter().map(|chs| hash(&chs)).sum()
}

fn solve_part2(seq: &Vec<&str>) -> usize {
    let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];

    for lens in seq {
        let label = find_label(&lens);
        let lenses = &mut boxes[hash(label) as usize];
        let existing_label_pos = lenses.iter().position(|l| find_label(&l) == label);

        if lens.chars().last().unwrap() == '-' {
            if let Some(n) = existing_label_pos {
                lenses.remove(n);
            }
        } else if let Some(n) = existing_label_pos {
            lenses[n] = lens;
        } else {
            lenses.push(lens);
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, c)| {
                    (i + 1) * (j + 1) * c.chars().last().unwrap().to_digit(10).unwrap() as usize
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn run(input: String) {
    let seq: Vec<_> = input.trim().split(',').collect();
    let part1_result = solve_part1(&seq);
    let part2_result = solve_part2(&seq);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}
