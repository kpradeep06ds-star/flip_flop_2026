pub fn movement(arr:char, mut idx:i32) -> i32 {
    // let mut idx = 0;
    if arr == '>'{
        idx += 1;
        idx = idx.rem_euclid(100);
    } else{
        idx -= 1;
        idx = idx.rem_euclid(100);    
    }
    idx
}


pub fn part_b(text:String) -> i32{
    let org_text = text.clone().chars().collect::<Vec<char>>();
    let rev_text = text.clone().chars().rev().collect::<Vec<char>>();
    let length = org_text.len();
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut res = 0;
    for i in 0..length {
        idx1 = movement(org_text.clone()[i], idx1 as i32);
        idx2 = movement(rev_text.clone()[i], idx2 as i32);
        if idx1 == idx2{
            res += 1;
        }
    }
    res
}