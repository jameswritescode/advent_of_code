pub fn run(input: String) {
    let mut result = 0;

    for line in input.lines() {
        let data: Vec<_> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let count = data.first().unwrap() * 10 + data.last().unwrap();

        result += count;
    }

    println!("Answer: {result}")
}
