use regex::Regex;

pub fn process_part1(input: &str) -> String {
    let (initial_order, operations) = input.split_once("\n\n").unwrap();
    let mut initial_order: Vec<&str> = initial_order.split("\n").collect();
    let operations: Vec<&str> = operations.split("\n").collect();

    let num = initial_order.pop().unwrap();
    let num: Vec<_> = num.split(" ").filter(|&entry| !entry.is_empty()).collect();
    let width = num.len();

    let re = Regex::new(r"\[(\w)\]").unwrap();

    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(width);
    for _ in 0..width {
        stacks.push(Vec::new());
    }

    for &line in initial_order.iter().rev() {
        for mat in re.find_iter(line) {
            let col = mat.start() / 4;
            stacks[col].push(line.chars().nth(mat.start() + 1).unwrap().to_string());
        }
    }

    let re = Regex::new(r"(\d+)").unwrap();
    for &operation in operations.iter() {
        let mut ops: Vec<usize> = Vec::with_capacity(3);
        
        for mat in re.find_iter(operation) {
            let num_parsed = operation[mat.start()..mat.end()].parse::<usize>().unwrap();
            ops.push(num_parsed);
        }
        let num = ops[0];
        let from = ops[1] - 1;
        let to = ops[2] - 1;

        let idx = stacks[from].len() - num;

        let mut removed = stacks[from].split_off(idx).into_iter().rev().collect();
        stacks[to].append(&mut removed);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
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
