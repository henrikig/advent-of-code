use std::{num::ParseIntError, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        return Move {
            num: iter.next().expect("num doesn't exist"),
            from: iter.next().expect("from doesn't exist") - 1,
            to: iter.next().expect("to doesn't exist") - 1,
        };
    }
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `move 3 from 1 to 2` -> Move { num: 3, from: 1, to: 2 }
        return Ok(s
            .split_whitespace()
            .filter_map(|op| op.parse::<usize>().ok())
            .collect::<Move>());
    }
}

pub fn process_part1(input: &str) -> String {
    // Separate crates from moves
    let (initial_order, operations) = input.split_once("\n\n").unwrap();
    let mut initial_order: Vec<&str> = initial_order.split("\n").collect();
    let operations: Vec<&str> = operations.split("\n").collect();

    // Get the number of horizontal stacks
    let num_stacks = initial_order.pop().unwrap();
    let num_stacks: Vec<_> = num_stacks
        .split(" ")
        .filter(|&entry| !entry.is_empty())
        .collect();
    let num_stacks = num_stacks.len();

    // Regex to match on `[<character>]`
    let re = Regex::new(r"\[(\w)\]").unwrap();

    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // Match on crates and push them to stacks
    for &line in initial_order.iter().rev() {
        for mat in re.find_iter(line) {
            let col = mat.start() / 4;
            stacks[col].push(line.chars().nth(mat.start() + 1).unwrap().to_string());
        }
    }

    // Parse the operations and perform them
    for &operation in operations.iter() {
        let mv: Move = operation
            .parse::<Move>()
            .expect(&format!("could not parse operation {}", operation));
        let idx = stacks[mv.from].len() - mv.num;

        let mut removed = stacks[mv.from].split_off(idx).into_iter().rev().collect();
        stacks[mv.to].append(&mut removed);
    }

    stacks
        .iter()
        .map(|stack| match stack.iter().last() {
            Some(s) => s,
            None => "",
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = process_part1(input);
        assert_eq!(result, "CMZ");
    }
}
