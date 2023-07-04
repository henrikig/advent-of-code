use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
struct Node {
    value: i32,
    children: Vec<Node>,
}

fn bfs(root: Node, target: i32) -> Option<Node> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        if node.value == target {
            return Some(node);
        }

        visited.insert(node.value);

        for child in &node.children {
            if !visited.contains(&child.value) {
                queue.push_back(child.clone());
            }
        }
    }

    None
}

fn main() {
    // Example tree
    let tree = Node {
        value: 1,
        children: vec![
            Node {
                value: 2,
                children: vec![
                    Node {
                        value: 4,
                        children: vec![],
                    },
                    Node {
                        value: 5,
                        children: vec![],
                    },
                ],
            },
            Node {
                value: 3,
                children: vec![
                    Node {
                        value: 6,
                        children: vec![],
                    },
                    Node {
                        value: 7,
                        children: vec![],
                    },
                ],
            },
        ],
    };

    let target = 6;
    if let Some(node) = bfs(tree, target) {
        println!("Node with value {} found!", node.value);
    } else {
        println!("Node with value {} not found.", target);
    }
}
