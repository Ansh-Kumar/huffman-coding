// pub mod huffman;

// use std::collections::BinaryHeap;
use std::collections::HashMap;

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
}
