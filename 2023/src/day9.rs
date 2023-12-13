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

fn solve_part1(input: &String) -> i32 {
    input
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

            stack.iter().fold(0, |acc, v| acc + v.last().unwrap())
        })
        .sum()
}

pub fn run(input: String) {
    let part1_result = solve_part1(&input);

    println!("Part 1: {part1_result}")
}
