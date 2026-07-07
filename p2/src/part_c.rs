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

pub fn part_c(text:String) -> i32{
    let org_text = text.clone().chars().collect::<Vec<char>>();
    // org_text will keep the pointer check
    let rev_text = text.clone().chars().rev().collect::<Vec<char>>();
    // index will increase basis of robot
    // 1 2 3 4 5 
    // for R will be move 1 place fowrad then backward ><>> ... >>>>, -> pointer check
    // but since reverse of ><>>...>>>> is >>>> it means the index will move ahead -> index check 
    let length = org_text.len();
    let mut idx = 0;
    let mut pointer = 0;
    let mut v:Vec<i32> = vec![0;100];
    for i in 0..length{
        idx = movement(org_text[i], idx); 
        pointer = movement(rev_text[i], pointer);
        let segment = (idx - pointer).rem_euclid(100);
        v[segment as usize] += 1;
    }

    let max_element = v.iter()
    .enumerate()
    .max_by(|(idx_a, a), (idx_b, b)| {
        a.partial_cmp(b)
            .unwrap()
            .then_with(|| idx_b.cmp(idx_a)) 
    });

    if let Some((index, value)) = max_element {
        println!("Max value is {} at index {}", value, index);
        return (index as i32 + 1)*value;
    } else{
        return 1;
    } 

}