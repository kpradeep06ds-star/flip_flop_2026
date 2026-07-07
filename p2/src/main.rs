use p2::part_a::part_a;
use p2::part_b::part_b;
use p2::part_c::part_c;

use std::fs;

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("FileNotFound");
    let out_a = part_a(text.clone());
    let out_b = part_b(text.clone());
    let out_c = part_c(text.clone());
    println!("{out_a} {out_b} {out_c}");
}
