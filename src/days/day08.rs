//! ## --- Day 8: Memory Maneuver ---
//!
//! TIL: a little bit more about Rust memory layout.

#[derive(Default)]
pub struct Node {
    pub children: Vec<Node>,
    pub meta: Vec<u32>,
}

fn parse_tree(data: &mut Vec<u32>) -> Node {
    let mut children_count = data.pop().unwrap();
    let mut meta_count = data.pop().unwrap();

    let mut current_node = Node::default();

    while children_count != 0 {
        children_count -= 1;
        current_node.children.push(parse_tree(data));
    }

    while meta_count != 0 {
        meta_count -= 1;
        current_node.meta.push(data.pop().unwrap());
    }

    current_node
}

pub fn parse(input: &str) -> Node {
    #[rustfmt::skip]
    let mut numbers: Vec<u32> =
        input.split_whitespace()
        .map(|n| n.parse().unwrap())
        .rev()
        .collect();

    parse_tree(&mut numbers)
}

/// What is the sum of all metadata entries?
pub fn part1(root: &Node) -> u32 {
    let mut meta_sum = 0;
    let mut to_visit = vec![];

    to_visit.push(root);

    while let Some(node) = to_visit.pop() {
        meta_sum += node.meta.iter().sum::<u32>();
        to_visit.extend(&node.children);
    }

    meta_sum
}

fn get_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        return node.meta.iter().sum();
    }

    let mut value = 0;

    for &node_idx in &node.meta {
        if node_idx != 0 && node_idx <= node.children.len() as u32 {
            value += get_value(&node.children[node_idx as usize - 1]);
        }
    }

    value
}

/// What is the value of the root node?
pub fn part2(root: &Node) -> u32 {
    get_value(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 138);
        assert_eq!(part2(&input_data), 66);
    }
}
