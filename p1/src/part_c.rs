use std::{collections::HashMap};

pub fn helper(text:String) -> HashMap<usize, Vec<i32>> {
    
    let lines_count = text.lines().collect::<Vec<&str>>().len();
    let mod_half = lines_count/2;
    let mut hmap:HashMap<usize, Vec<i32>> = HashMap::new();

    for (idx, i) in text.lines().enumerate(){
        let j = i.parse::<i32>().unwrap();
        hmap.entry(idx % mod_half).or_default().push(j);
    }

    hmap
}

pub fn part_c(text:String) -> i32 {
    let hmap = helper(text);
    let mut diffvalue;
    let mut total_diff = 0;
    for i in hmap.values(){
        
        diffvalue = i[1] as i32 - i[0];
        //println!("{diffvalue}");
        if diffvalue > 0 {
            total_diff += diffvalue;
        } else {
            total_diff += (-1)*diffvalue*5;
        }
    }
    total_diff
}