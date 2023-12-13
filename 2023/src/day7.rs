struct Hand {
    bet: u32,
    score: u32,
}

type Hands = Vec<Hand>;

// h/t https://is.gd/f51lZu - had to learn more about bitwise operations.
fn from_str(s: &String, with_jokers: bool) -> Hands {
    let mut hands: Vec<_> = s
        .lines()
        .map(|l| {
            let mut cards = [0; 15];
            let mut parts = l.split_whitespace();
            let mut score = 0;
            let mut jokers = 0;

            for (i, card) in parts.next().unwrap().chars().enumerate() {
                let card_score = match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if with_jokers {
                            1
                        } else {
                            11
                        }
                    }
                    'T' => 10,
                    _ => card.to_digit(10).unwrap(),
                };

                if with_jokers && card == 'J' {
                    jokers += 1;
                } else {
                    cards[card_score as usize] += 1;
                }

                score |= card_score << (4 - i) * 4;
            }

            cards.sort_unstable();

            let hand_type = match cards[14] + jokers {
                5 => 6,
                4 => 5,
                3 if cards[13] == 2 => 4,
                3 => 3,
                2 if cards[13] == 2 => 2,
                2 => 1,
                _ => 0,
            };

            score |= hand_type << 20;

            Hand {
                score,
                bet: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    hands.sort_by_key(|h| h.score);

    hands
}

fn solve(hands: Hands) -> usize {
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bet as usize * (i + 1))
}

fn solve_part1(input: &String) -> usize {
    solve(from_str(&input, false))
}

fn solve_part2(input: &String) -> usize {
    solve(from_str(&input, true))
}

pub fn run(input: String) {
    let part1_result = solve_part1(&input);
    let part2_result = solve_part2(&input);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
