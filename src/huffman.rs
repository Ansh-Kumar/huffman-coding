// pub mod huffman;

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
pub struct HuffmanTree {
    head: Option<HuffmanTreeNode>,
}

#[derive(Debug)]
struct HuffmanTreeNode {
    key: Option<char>,
    freq: i64,
    left: Box<Option<HuffmanTreeNode>>,
    right: Box<Option<HuffmanTreeNode>>,
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl Eq for HuffmanTreeNode {}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

// get a list of character frequencies
pub fn assign_freq(input: &str) -> HashMap<char, i64> {
    let mut freq: HashMap<char, i64> = HashMap::new();
    for c in input.chars() {
        match freq.get(&c) {
            Some(v) => freq.insert(c, v + 1),
            None => freq.insert(c, 1),
        };
    }
    freq
}

impl HuffmanTreeNode {
    fn new(key: Option<char>, freq: i64) -> Self {
        HuffmanTreeNode {
            key,
            freq,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

impl Default for HuffmanTreeNode {
    fn default() -> Self {
        HuffmanTreeNode {
            // Provide default values for the fields
            key: None,
            freq: 0,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

pub fn build_tree(freq_map: HashMap<char, i64>) -> HuffmanTree {
    let mut min_heap: BinaryHeap<Reverse<HuffmanTreeNode>> = BinaryHeap::new();

    // find the two min values

    let mut node;

    for (key, &value) in &freq_map {
        node = HuffmanTreeNode::new(Some(*key), value);
        min_heap.push(Reverse(node));
    }

    let mut left;
    let mut right;
    let mut total_freq: i64;
    let mut new_node: HuffmanTreeNode;

    while min_heap.len() > 1 {
        left = min_heap.pop().unwrap().0;
        right = min_heap.pop().unwrap().0;
        println!("left: {:#?}\nright: {:#?}", left, right);
        total_freq = left.freq + right.freq;

        new_node = HuffmanTreeNode::new(None, total_freq);
        new_node.left = Box::new(Some(left));
        new_node.right = Box::new(Some(right));

        min_heap.push(Reverse(new_node));
    }

    HuffmanTree {
        head: Some(min_heap.pop().unwrap().0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_freq_basic_test() {
        let s: &str = "Hello";
        let freq = assign_freq(s);
        let correct = HashMap::from([('H', 1), ('e', 1), ('l', 2), ('o', 1)]);
        assert_eq!(freq, correct);
    }

    #[test]
    fn build_tree_basic_test() {
        let s: &str = "Hello";
        let freq = assign_freq(s);
        let tree = build_tree(freq);
        assert_eq!(tree.head.unwrap().freq, 5);
    }

    #[test]
    fn build_tree_intermediate_test() {
        let s: &str = "AABCBDEABBDC";
        let l = s.len() as i64;
        let freq = assign_freq(s);
        let tree = build_tree(freq);

        assert_eq!(tree.head.unwrap().freq, l);
    }
}
