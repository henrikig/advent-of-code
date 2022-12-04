use std::{char, collections::HashSet};

fn num_to_priotity(num: u32) -> u32 {
    match num {
        u32::MIN..=90 => num - 38,
        91..=u32::MAX => num - 96,
    }
}

pub fn process_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<char> = HashSet::from_iter(first.chars());
            let second: HashSet<char> = HashSet::from_iter(second.chars());

            let num = *first.intersection(&second).next().unwrap() as u32;
            num_to_priotity(num)
        })
        .sum::<u32>();
    result
}

pub fn process_part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let result = lines
        .chunks(3)
        .map(|chunk| {
            let mut sets: Vec<HashSet<char>> = Vec::new();
            sets.push(HashSet::from_iter(chunk[0].chars()));
            sets.push(HashSet::from_iter(chunk[1].chars()));
            sets.push(HashSet::from_iter(chunk[2].chars()));

            let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

            let num = *intersection.iter().next().unwrap() as u32;
            num_to_priotity(num)
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn split_middle() {
        let c = 'A';
        println!("{}", c as u32 - 38);
        let c = 'Z';
        println!("{}", c as u32);
        let c = 'a';
        println!("{}", c as u32 - 96);
        let c = 'z';
        println!("{}", c as u32 - 96);
    }

    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = process_part1(input);
        assert_eq!(result, 157);
    }
}
