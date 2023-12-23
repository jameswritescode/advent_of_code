pub fn run(input: String) {
    let part1_result: u32 = input
        .trim()
        .split(',')
        .map(|chs| {
            chs.chars()
                .map(|c| c as u32)
                .fold(0, |acc, n| ((acc + n) * 17) % 256)
        })
        .sum();

    println!("Part 1: {}", part1_result);
}
