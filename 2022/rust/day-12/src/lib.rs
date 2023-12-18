use std::{fmt::Display, str::FromStr};

struct Grid {
    grid: Vec<Vec<usize>>,
    start: usize,
    end: usize,
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Grid, &'static str> {
        let lines: Vec<&str> = input.lines().collect();
        let mut grid = Vec::with_capacity(lines.len());
        let mut start = None;
        let mut end = None;
        for (row_num, line) in lines.iter().enumerate() {
            let length = line.len();
            let mut row = Vec::with_capacity(length);
            for (col, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        row.push('a' as usize - 'a' as usize);
                        start = Some(row_num * length + col);
                    }
                    'E' => {
                        row.push('z' as usize - 'a' as usize);
                        end = Some(row_num * length + col);
                    }
                    'a'..='z' => row.push(c as usize - 'a' as usize),
                    _ => (),
                }
            }
            grid.push(row);
        }
        return Ok(Grid {
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
        });
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{:2} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Node(u32);

struct Edge(u32, u32);

struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(edges: Vec<Edge>) -> Self {
        Graph { edges }
    }
}

impl From<Grid> for Graph {
    fn from(grid: Grid) -> Self {
        todo!();
    }
}

pub fn process_part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    println!("{}", grid);
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
