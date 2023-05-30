use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Clone)]
struct Monkey {
    items: Vec<i32>,
    monkey_fn: Box<dyn Fn(&Self) -> i32>,
    divisible_by: i32,
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Skip monkey number
        let mut lines = s.lines().skip(1);

        // Parse starting items: Starting items: 79, 98
        let items: Vec<i32> = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<i32>().unwrap())
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

        let apply_fn = match operands[1] {
            "*" => |a: i32, b: i32| a * b,
            "+" => |a: i32, b: i32| a + b,
            _ => panic!("Invalid operation"),
        };

        let monkey_fn: Box<dyn Fn(&Monkey) -> i32> = match operands[2].parse::<i32>() {
            Ok(n) => Box::new(move |monkey: &Monkey| apply_fn(monkey.items[0], n)),
            Err(_) => Box::new(move |monkey: &Monkey| apply_fn(monkey.items[0], monkey.items[0])),
        };

        // Parse test: Test: divisible by 23
        let divisible_by = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        Ok(Monkey {
            items,
            monkey_fn,
            divisible_by,
            inspection_count: 0,
        })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.items, self.inspection_count)
    }
}

impl Monkey {
    fn is_divisible(&self) -> bool {
        self.items[0] % self.divisible_by == 0
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
            let monkey = self.monkeys[i].clone();
            while !monkey.items.is_empty() {
                let item = monkey.items.remove(0);
                self.monkeys[2].items.push(item);
            }
        }
    }
}

pub fn process_part1(input: &str) -> usize {
    let monkey_game = MonkeyGame::new(input);

    for monkey in monkey_game.monkeys.iter() {
        println!("{}", monkey);
        println!(
            "{} divisible by {}: {}",
            monkey.items[0],
            monkey.divisible_by,
            monkey.is_divisible()
        );
    }

    return 4;
}

pub fn process_part2(input: &str) -> usize {
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
        assert_eq!(result, 4);
    }
}
