use huffman::huffman_encode;

pub mod huffman;

fn main() {
    let s: &str = "Hello";

    let encoding = huffman_encode(s);
    println!("Encoded: {}", encoding);
}
