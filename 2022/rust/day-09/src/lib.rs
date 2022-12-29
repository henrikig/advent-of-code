use std::{collections::HashSet, num::ParseIntError, ops::AddAssign, str::FromStr};

#[derive(Debug)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, count) = s.split_once(" ").unwrap();

        let count = count.parse::<usize>()?;

        match direction {
            "U" => Ok(Self::Up(count)),
            "D" => Ok(Self::Down(count)),
            "L" => Ok(Self::Left(count)),
            "R" => Ok(Self::Right(count)),
            _ => unreachable!(),
        }
    }
}

impl Move {
    fn get_direction(&self) -> Position {
        match self {
            Move::Up(_) => Position(0, 1),
            Move::Down(_) => Position(0, -1),
            Move::Left(_) => Position(-1, 0),
            Move::Right(_) => Position(1, 0),
        }
    }

    fn get_count(&self) -> usize {
        match self {
            Move::Up(count) => *count,
            Move::Down(count) => *count,
            Move::Left(count) => *count,
            Move::Right(count) => *count,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Position(i32, i32);

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Position(self.0 + other.0, self.1 + other.1);
    }
}

impl Position {
    fn follow_head(&mut self, head: &Self) {
        let x_diff = head.0 - self.0;
        let y_diff = head.1 - self.1;

        let x_move = match x_diff {
            i32::MIN..=-1 => -1,
            0 => 0,
            1..=i32::MAX => 1,
        };

        let y_move = match y_diff {
            i32::MIN..=-1 => -1,
            0 => 0,
            1..=i32::MAX => 1,
        };

        self.0 += x_move;
        self.1 += y_move;
    }
}

fn euclidean_dist(head: &Position, tail: &Position) -> f64 {
    let x_diff = (head.0 - tail.0).pow(2);
    let y_diff = (head.1 - tail.1).pow(2);

    let sum = x_diff + y_diff;
    return (sum as f64).sqrt();
}

fn get_moves_from_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|m| Move::from_str(m).expect("Could not parse move"))
        .collect::<Vec<Move>>()
}

pub fn process_part1(input: &str) -> usize {
    let moves = get_moves_from_input(input);

    let mut seen: HashSet<Position> = HashSet::new();
    let mut head_pos = Position(0, 0);
    let mut tail_pos = Position(0, 0);
    seen.insert(tail_pos);

    for mv in moves {
        let direction = mv.get_direction();
        let count = mv.get_count();
        for _ in 0..count {
            head_pos += direction;
            let dist = euclidean_dist(&head_pos, &tail_pos);
            if dist >= 2.0 {
                tail_pos.follow_head(&head_pos);
                seen.insert(tail_pos);
            }
        }
    }
    return seen.len();
}

pub fn process_part2(input: &str) -> usize {
    const SNAKE_LEN: usize = 10;
    let moves = get_moves_from_input(input);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut snake = vec![Position(0, 0); SNAKE_LEN];
    visited.insert(*snake.last().expect("Snake is empty"));

    for mv in moves {
        let direction = mv.get_direction();
        let count = mv.get_count();
        for _ in 0..count {
            let mut temp_snake = vec![];
            *snake.first_mut().expect("Snake has no head..") += direction;
            temp_snake.push(*snake.first().unwrap());
            for (i, _) in snake[..SNAKE_LEN - 1].iter().enumerate() {
                let head = temp_snake.last().expect("Temp snake should have values...");
                let mut tail = snake[i + 1];
                let dist = euclidean_dist(&head, &tail);
                if dist >= 2.0 {
                    tail.follow_head(&head);
                }
                temp_snake.push(tail);
            }
            visited.insert(*temp_snake.last().expect("Snake is empty..."));
            snake = temp_snake;
        }
        dbg!(&snake);
    }
    return visited.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = process_part2(input);
        assert_eq!(result, 36);
    }
}
