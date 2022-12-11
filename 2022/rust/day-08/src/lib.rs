pub fn day_08(input: &str) -> u32 {
    let mut grid = vec![];

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        grid.push(row);
    }

    let length = grid.len();
    let mut num_visible = length as u32 * 4 - 4;

    // We dont need to check outer
    // For all inner, check if all trees left of, right of, over and under are lower

    for (i, row) in grid.iter().enumerate() {
        for (j, current_tree) in row.iter().enumerate() {
            if is_edge_tree(i, j, length) {
                continue;
            }

            if grid[i][0..j]
                .iter()
                .all(|neighbour_tree| neighbour_tree < current_tree)
                || grid[i][j + 1..]
                    .iter()
                    .all(|neighbour_tree| neighbour_tree < current_tree)
                || grid[0..i]
                    .iter()
                    .all(|trees_above| trees_above[j] < *current_tree)
                || grid[i + 1..]
                    .iter()
                    .all(|trees_below| trees_below[j] < *current_tree)
            {
                num_visible += 1;
            }
        }
    }
    num_visible
}

pub fn process_part2(input: &str) -> u32 {
    let mut grid = vec![];

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        grid.push(row);
    }

    let length = grid.len();

    let mut current_best = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, current_tree) in row.iter().enumerate() {
            if is_edge_tree(i, j, length) {
                continue;
            }

            let left_viewing_distance = grid[i][0..j]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, &neighbour_tree)| neighbour_tree >= *current_tree)
                // if no trees are higher, we can see all the way to tree 0
                .unwrap_or((j - 1, &0))
                .0
                + 1;

            if left_viewing_distance == 0 {
                continue;
            }

            let right_viewing_distance = grid[i][j + 1..]
                .iter()
                .enumerate()
                .find(|(_, &neighbour_tree)| neighbour_tree >= *current_tree)
                // if no trees are higher, we can see all the way to tree 0
                .unwrap_or((length - j - 2, &0))
                .0
                + 1;

            if right_viewing_distance == 0 {
                continue;
            }

            let upward_viewing_distance = grid[0..i]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, trees_above)| trees_above[j] >= *current_tree)
                .unwrap_or((i - 1, &vec![0]))
                .0
                + 1;

            if upward_viewing_distance == 0 {
                continue;
            }

            let downward_viewing_distance = grid[i + 1..]
                .iter()
                .enumerate()
                .find(|(_, trees_below)| trees_below[j] >= *current_tree)
                .unwrap_or((length - i - 2, &vec![0]))
                .0
                + 1;

            let view_score = left_viewing_distance
                * right_viewing_distance
                * upward_viewing_distance
                * downward_viewing_distance;

            if view_score > current_best {
                current_best = view_score;
            }
        }
    }

    current_best as u32
}

fn is_edge_tree(i: usize, j: usize, length: usize) -> bool {
    return i == 0 || j == 0 || i == length - 1 || j == length - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "30373
25512
65332
33549
35390";
        let result = process_part2(input);
        assert_eq!(result, 21);
    }
}
