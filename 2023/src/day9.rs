fn differences(numbers: &Vec<i32>) -> Vec<i32> {
    let mut list = vec![];

    for (i, number) in numbers.iter().enumerate() {
        let next = i + 1;

        if next == numbers.len() {
            break;
        }

        list.push(numbers[next] - number);
    }

    list
}

type Stacks = Vec<Vec<Vec<i32>>>;

fn solve_part1(stacks: &Stacks) -> i32 {
    stacks
        .iter()
        .map(|s| s.iter().fold(0, |acc, v| acc + v[v.len() - 1]))
        .sum()
}

fn solve_part2(stacks: &Stacks) -> i32 {
    stacks
        .iter()
        .map(|s| s.iter().rev().fold(0, |acc, v| v[0] - acc))
        .sum()
}

pub fn run(input: String) {
    let stacks: Stacks = input
        .lines()
        .map(|l| {
            let list = l
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut stack: Vec<_> = vec![list];

            while !stack.last().unwrap().iter().all(|&c| c == 0) {
                stack.push(differences(stack.last().unwrap()));
            }

            stack
        })
        .collect();

    let part1_result = solve_part1(&stacks);
    let part2_result = solve_part2(&stacks);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
