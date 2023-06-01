use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Clone, Debug)]
enum Operation {
    Multiply,
    Add,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Clone, Debug)]
enum Term {
    Old,
    Constant(i64),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operand: Operation,
    term: Term,
    divisible_by: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

impl Monkey {
    fn apply_operation(&self, old: i64) -> i64 {
        let new = match self.term {
            Term::Old => match self.operand {
                Operation::Multiply => old * old,
                Operation::Add => old + old,
            },
            Term::Constant(constant) => match self.operand {
                Operation::Multiply => old * constant,
                Operation::Add => old + constant,
            },
        };
        return new;
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Skip monkey number
        let mut lines = s.lines().skip(1);

        // Parse starting items: Starting items: 79, 98
        let items: Vec<i64> = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<i64>().unwrap())
            .collect();

        // Parse operation: Operation: new = old * 19
        let operands = lines
            .next()
            .unwrap()
            .split("= ")
            .nth(1)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>();

        let operand = operands[1].parse::<Operation>().unwrap();

        let term = match operands[2].parse::<i64>() {
            Ok(constant) => Term::Constant(constant),
            Err(_) => Term::Old,
        };

        // Parse test: Test: divisible by 23
        let divisible_by = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Parse true monkey: If true: throw to monkey 2
        let true_monkey = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // Parse false monkey: If false: throw to monkey 3
        let false_monkey = lines
            .next()
            .unwrap()
            .split("monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(Monkey {
            items,
            operand,
            term,
            divisible_by,
            true_monkey,
            false_monkey,
            inspection_count: 0,
        })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.items, self.inspection_count)
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

impl MonkeyGame {
    fn new(input: &str) -> Self {
        let monkeys = input
            .split("\n\n")
            .map(|line| Monkey::from_str(line).unwrap())
            .collect::<Vec<Monkey>>();

        MonkeyGame { monkeys }
    }

    fn play_round(&mut self) {
        let num_monkeys = self.monkeys.len();

        for i in 0..num_monkeys {
            let mc;
            {
                let monkey = &mut self.monkeys[i];
                mc = monkey.clone();
                monkey.inspection_count += mc.items.len();
            }

            for mut item in mc.items.iter().copied() {
                item = mc.apply_operation(item);
                // item /= 3;
                if item % mc.divisible_by == 0 {
                    self.monkeys[mc.true_monkey].items.push(item);
                } else {
                    self.monkeys[mc.false_monkey].items.push(item);
                }
            }
            self.monkeys[i].items.clear();
        }
    }
}

pub fn process_part1(input: &str) -> usize {
    let mut monkey_game = MonkeyGame::new(input);

    for i in 0..10000 {
        dbg!(i);
        monkey_game.play_round();
    }

    // get the product of the inspection count of the two monkies with the highest inspection count
    let mut inspection_counts = monkey_game
        .monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();

    inspection_counts.sort();
    inspection_counts.reverse();
    let product = inspection_counts[0] * inspection_counts[1];

    return product;
}

pub fn process_part2(input: &str) -> usize {
    dbg!(input);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let result = process_part1(input);
        assert_eq!(result, 10605);
    }
}
