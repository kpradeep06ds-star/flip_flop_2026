// use fancy_regex::Regex;
// use std::{cmp::max};
use crate::part_b::{one_digit_7, one_each, red_blue_green, repeats};

pub fn one_function_score(l:String) -> i32{

    let mut score ;
    let len = l.len() as i32;
    score = one_each(l.to_string());
        
    if one_digit_7(l.to_string()){
        score += 7;
    }

    let rep = repeats(l.to_string());
    
    if rep == 0{
        score = score*1;
    } else{
        score = score + rep*rep;
    }
    // println!("{score}");

    let rgb = red_blue_green(l.to_string());
    
    if rgb == true{
        score = score*3;
    }
    
    score = score*len;

    score

}

// find a 7 and don't find any other number
pub fn part_c(lines: String) -> i32{
    let mut scores ;
    let mut k:String ;
    let mut score_v:Vec<i32> = Vec::new();
    let alphanumeric: String = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .collect();
    for i in alphanumeric.chars() {
        scores = 0;
        for l in lines.lines(){
            k = l.to_string();
            k.push_str(&i.to_string());
            let value = one_function_score(k);    
            //println!("{value}");
            scores += value;
        }
        score_v.push(scores);
    }
    *score_v.iter().max().unwrap()
}