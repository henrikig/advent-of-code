use std::collections::HashSet;

pub fn process_part1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();

    let res = chars
        .windows(4)
        .enumerate()
        .find_map(|(i, window)| {
            let set: HashSet<&char> = HashSet::from_iter(window.iter());
            match set.len() {
                4 => Some(i + 4),
                _ => None,
            }
        })
        .expect("No sequences of four characters");

    res
}

pub fn process_part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();

    let res = chars
        .windows(14)
        .enumerate()
        .find_map(|(i, window)| {
            let set: HashSet<&char> = HashSet::from_iter(window.iter());
            match set.len() {
                14 => Some(i + 14),
                _ => None,
            }
        })
        .expect("No sequences of four characters");

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = process_part1(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = process_part1(input);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = process_part1(input);
        assert_eq!(result, 10);
    }
    #[test]
    fn it_works4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = process_part1(input);
        assert_eq!(result, 11);
    }
}
