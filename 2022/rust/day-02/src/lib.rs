pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|game| match game.chars().nth(0).unwrap() {
            'A' => match game.chars().nth(2).unwrap() {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => 0,
            },
            'B' => match game.chars().nth(2).unwrap() {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match game.chars().nth(2).unwrap() {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<i32>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|game| match game.chars().nth(0).unwrap() {
            'A' => match game.chars().nth(2).unwrap() {
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => 0,
            },
            'B' => match game.chars().nth(2).unwrap() {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0,
            },
            'C' => match game.chars().nth(2).unwrap() {
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "A Y
B X
C Z";
        let result = process_part1(input);
        assert_eq!(result, "15");
    }
}
