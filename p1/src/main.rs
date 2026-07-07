use std::fs;

use p1::part_a::part_a;
use p1::part_b::part_b;
use p1::part_c::part_c;

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("FileNotFound");
    let out_a = part_a(text.clone());
    let out_b = part_b(text.clone());
    let out_c = part_c(text.clone());
    println!("{:?}", out_a);
    println!("{:?}", out_b);
    println!("{:?}", out_c);
}
