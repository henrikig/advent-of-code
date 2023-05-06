use std::str::FromStr;

enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap();

        match op {
            "addx" => {
                let arg = parts.next().unwrap().parse::<i32>().unwrap();
                Ok(Instruction::AddX(arg))
            }
            "noop" => Ok(Instruction::Noop),
            _ => panic!("Unknown instruction"),
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect()
}

pub fn process_part1(input: &str) -> i32 {
    let instructions = get_instructions(input);
    let signal_cycles = [20, 60, 100, 140, 180, 220];
    let mut result = 0;
    let mut register = 1;
    let mut cycle = 0;
    for instruction in instructions.iter() {
        match instruction {
            Instruction::AddX(arg) => {
                for _ in 0..2 {
                    cycle += 1;
                    if signal_cycles.contains(&cycle) {
                        result += cycle * register;
                    }
                }
                register += arg;
            }
            Instruction::Noop => {
                cycle += 1;
                if signal_cycles.contains(&cycle) {
                    result += cycle * register;
                }
            }
        }
    }
    return result;
}

fn get_pos_from_num(num: i32) -> (usize, usize) {
    let row = (num - 1) / 40;
    let col = (num - 1) % 40;
    (row as usize, col as usize)
}

fn get_sprite_cols(sprite_center: usize) -> Vec<usize> {
    let left = match sprite_center.checked_sub(1) {
        Some(left) => left,
        None => usize::MAX,
    };
    let right = match sprite_center.checked_add(1) {
        Some(right) => right,
        None => usize::MAX,
    };
    return vec![left, sprite_center, right];
}

fn get_pixel_value(current_pixel: (usize, usize), sprite_center: (usize, usize)) -> &'static str {
    let (_, col) = current_pixel;
    let (_, sprite_col) = sprite_center;
    let sprite_cols = get_sprite_cols(sprite_col);
    if sprite_cols.contains(&col) {
        return "#";
    }
    return ".";
}

fn draw_pixel(pixels: &mut [[&str; 40]; 6], cycle: i32, sprite_center: i32) {
    let cycle_pos = get_pos_from_num(cycle);
    let sprite_pos = get_pos_from_num(sprite_center);
    dbg!(cycle_pos, sprite_pos, cycle);
    let pixel_value = get_pixel_value(cycle_pos, sprite_pos);
    pixels[cycle_pos.0][cycle_pos.1] = pixel_value;
}

pub fn process_part2(input: &str) {
    let instructions = get_instructions(input);
    let mut sprite_center = 2;
    let mut cycle = 0;
    let mut pixels: [[&str; 40]; 6] = [[""; 40]; 6];

    for instruction in instructions.iter() {
        match instruction {
            Instruction::AddX(arg) => {
                for _ in 0..2 {
                    cycle += 1;
                    draw_pixel(&mut pixels, cycle, sprite_center);
                }
                sprite_center += arg;
            }
            Instruction::Noop => {
                cycle += 1;
                draw_pixel(&mut pixels, cycle, sprite_center);
            }
        }
    }
    for row in pixels.iter() {
        let joined = row.join("");
        println!("{}", joined);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let result = process_part1(input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_prrocess_part2() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        process_part2(input);
    }
}
