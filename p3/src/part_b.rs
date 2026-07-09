use fancy_regex::Regex;
use std::{cmp::max};
// find a 7 and don't find any other number
pub fn one_digit_7(text: String) -> bool {
    let re1 = Regex::new(r"[012345689]").unwrap();
    let re2 = Regex::new(r"7").unwrap();
    !re1.is_match(&text).unwrap() && re2.is_match(&text).unwrap()
}

pub fn repeats(text:String) -> i32{
    let re = Regex::new(r"(\w)\1\1+").unwrap();
    let mut max_count = 0;
    // Iterate over all matches found in the text
    for mats in re.find_iter(&text) {
        let matched_str = mats.unwrap().as_str();
        let count = matched_str.chars().count();
        max_count = max(max_count, count);
        //return max_count as i32;
    }

    return max_count as i32; 

}

pub fn red_blue_green(text:String) -> bool{
    let re = Regex::new(r"blue|red|green").unwrap();
    re.is_match(&text).unwrap()
}

pub fn one_each(text:String) -> i32{
    let re1 = Regex::new(r"[A-Z]+").unwrap();
    let re2 = Regex::new(r"[a-z]+").unwrap();
    let re3 = Regex::new(r"\d+").unwrap();

    let one = re1.is_match(&text).unwrap() as i32;
    let two = re2.is_match(&text).unwrap() as i32;
    let three = re3.is_match(&text).unwrap() as i32;

    one + two + three
}

pub fn part_b(lines: String) -> (i32, String){
    
    // find a number if only 7 then add 7
    // find a number if present 1
    // fina a alpahbet cap - 1
    // find a alphabet small - 1
    // find >=3 digit square of number of repeats
    // red green blue x 3
    let mut score ;
    let mut maxscore: i32 = 0;
    let mut stringvalue = "a";
    
    for l in lines.lines(){
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

        if score >= maxscore{
            maxscore = score;
            stringvalue = l;
        }
        // println!("{score} {l}");
    }
    (maxscore, stringvalue.to_string())
}