//! ## --- Day 8: Memory Maneuver ---
//!

#[derive(Default, Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub meta: Vec<u32>,
}

fn parse_tree(data: &mut Vec<u32>) -> Node {
    let mut children_count = data.pop().unwrap();
    let mut meta_count = data.pop().unwrap();

    let mut current = Node::default();

    while children_count != 0 {
        children_count -= 1;
        current.children.push(parse_tree(data));
    }

    while meta_count != 0 {
        meta_count -= 1;
        current.meta.push(data.pop().unwrap());
    }

    current
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

pub fn part2(root: &Node) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 138);
    }
}
