use std::fs;
use p4::{part_a::replace_funny_chars, part_b::swap_branches};
// use p3::part_b::part_b;
// use p3::part_c::part_c;


fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("FileNotFound");
    println!("{:?}",replace_funny_chars(text.clone(), 400));
    println!("{:?}",swap_branches(text.clone()));
}
