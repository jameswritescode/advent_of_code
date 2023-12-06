use std::{collections::HashSet, convert::Infallible, str::FromStr};

struct Card {
    matches: usize,
}

fn collect_numbers(s: &str) -> Vec<u32> {
    s.split(" ").filter_map(|n| n.parse().ok()).collect()
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ").nth(1).unwrap().split("|");

        let winners: HashSet<_> = collect_numbers(parts.next().unwrap()).into_iter().collect();
        let numbers = collect_numbers(parts.next().unwrap());
        let matches = numbers.iter().filter(|i| winners.contains(i)).count();

        Ok(Card { matches })
    }
}

type Cards = Vec<Card>;

fn solve_part1(cards: &Cards) -> usize {
    cards
        .iter()
        .map(|c| {
            if c.matches == 0 {
                0
            } else {
                2usize.pow((c.matches - 1) as u32)
            }
        })
        .sum()
}

// h/t https://is.gd/OsjzYI
fn solve_part2(cards: &Cards) -> usize {
    let mut counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let j = card.matches + i;

        for k in i + 1..j + 1 {
            counts[k] += counts[i];
        }
    }

    counts.iter().sum()
}

pub fn run(input: String) {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    let part1_result = solve_part1(&cards);
    let part2_result = solve_part2(&cards);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
