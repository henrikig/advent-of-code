fn contains_other(input: &Vec<u32>, other: &Vec<u32>) -> bool {
    input[0] <= other[0] && input[1] >= other[1]
}

fn partial_overlap(input: &Vec<u32>, other: &Vec<u32>) -> bool {
    (other[0] >= input[0] && other[0] <= input[1]) || (other[1] >= input[0] && other[1] <= input[1])
}

pub fn process_part1(input: &str) -> u32 {
    let result = input
        .lines()
        .flat_map(|line| {
            line.split(",").map(|range| {
                // &"2-4"
                let mut range_iter = range.split("-");
                let start = range_iter.next().unwrap().parse::<u32>().unwrap();
                let end = range_iter.next().unwrap().parse::<u32>().unwrap();
                vec![start, end]
            })
        })
        .collect::<Vec<Vec<u32>>>();

    let result = result
        .chunks(2)
        .map(|chunk| {
            let first = &chunk[0];
            let second = &chunk[1];
            contains_other(first, second) || contains_other(second, first)
        })
        .map(|overlaps| if overlaps { 1 } else { 0 })
        .sum::<u32>();
    result
}

pub fn process_part2(input: &str) -> u32 {
    let result = input
        .lines()
        .flat_map(|line| {
            line.split(",").map(|range| {
                // &"2-4"
                let mut range_iter = range.split("-");
                let start = range_iter.next().unwrap().parse::<u32>().unwrap();
                let end = range_iter.next().unwrap().parse::<u32>().unwrap();
                vec![start, end]
            })
        })
        .collect::<Vec<Vec<u32>>>();

    let result = result
        .chunks(2)
        .map(|chunk| {
            let first = &chunk[0];
            let second = &chunk[1];
            partial_overlap(first, second) || partial_overlap(second, first)
        })
        .map(|overlaps| if overlaps { 1 } else { 0 })
        .sum::<u32>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = process_part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn iit_works() {
        let result = "2-4";
        let mut it = result.chars();
        it.next();
        it.skip(1).next();
    }
}
