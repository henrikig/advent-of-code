use std::fmt::Display;

struct Maze {
    maze: Vec<Vec<usize>>,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl Maze {
    pub fn new(input: &str) -> Result<Maze, &'static str> {
        let mut maze = Vec::with_capacity(input.len());
        let mut start = None;
        let mut end = None;
        for (row_num, line) in input.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        row.push('a' as usize - 'a' as usize);
                        start = Some((row_num, col));
                    }
                    'E' => {
                        row.push('z' as usize - 'a' as usize);
                        end = Some((row_num, col));
                    }
                    'a'..='z' => row.push(c as usize - 'a' as usize),
                    _ => (),
                }
            }
            maze.push(row);
        }
        if start.is_none() || end.is_none() {
            return Err("No start or end found");
        }
        Ok(Maze { maze, start, end })
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.maze {
            for col in row {
                write!(f, "{:2} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn process_part1(input: &str) -> usize {
    let maze = Maze::new(input).unwrap();
    println!("{}", maze);
    return 32;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn it_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, 31);
    }
}
