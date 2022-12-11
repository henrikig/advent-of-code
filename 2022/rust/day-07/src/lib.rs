pub fn day_07(input: &str) -> i32 {
    // We don't care about the first line (starting at root)
    let lines = input.split("\n").skip(1);
    let mut current_folders = vec![("/", 0)];
    let mut folder_sizes = vec![];

    for line in lines {
        if line == "$ ls" || line.starts_with("dir") {
            continue;
        } else if line == "$ cd .." {
            // Add folder size of escaped folder to final folder sizes
            let (_, file_size) = current_folders.pop().expect("Should be more folders...");
            folder_sizes.push(file_size);
            // Add folder size to parent
            current_folders.last_mut().unwrap().1 += file_size;
        } else if line.starts_with("$ cd") {
            let folder = line.split(" ").last().unwrap();
            current_folders.push((folder, 0));
        } else {
            // We have a file, add size to current folder
            let mut line = line.split(" ");
            let file_size = line
                .next()
                .expect("Should be a number here...")
                .parse::<i32>()
                .expect("Could not parse number");

            current_folders.last_mut().unwrap().1 += file_size;
        }
    }

    while current_folders.len() > 0 {
        let (_, file_size) = current_folders.pop().unwrap();
        folder_sizes.push(file_size);

        if current_folders.len() > 0 {
            current_folders.last_mut().unwrap().1 += file_size;
        }
    }

    let total_space = 70_000_000;
    let available_space = total_space - folder_sizes.last().unwrap();
    let to_be_deleted = 30_000_000 - available_space;

    //     Part 1 answer
    //     folder_sizes
    //         .iter()
    //         .filter(|&size| size <= &100_000)
    //         .sum::<i32>();

    // Part 2 answer
    *folder_sizes
        .iter()
        .filter(|&size| size >= &to_be_deleted)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = day_07(input);
        println!("Result {}", result);
        assert_eq!(result, 95437);
    }
}
