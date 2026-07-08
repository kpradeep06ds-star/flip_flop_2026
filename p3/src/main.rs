use std::fs;
use p3::part_a::part_a;


fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("FileNotFound");
    let out_a = part_a(text);
    println!("{:?}", out_a);
}
