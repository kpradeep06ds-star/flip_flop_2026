pub fn part_a(text:String) -> i32 {
//index min
// value , multiply them
    //let _t = text.chars().collect::<Vec<char>>();
    let mut v:Vec<i32> = vec![0;100];
    let mut idx:i32 = 0;

    for t in text.chars(){
        // println!("{t}");
        if t == '>'{
            idx += 1;
            if idx >= 100{
                idx = 100 - idx;
            } else if idx < 0{
                idx = 100 + idx
            }
            v[idx as usize] += 1;
           // println!("{} + {}",v[idx as usize],idx);
        } else {
            idx -= 1;
            if idx >= 100{
                idx = 100 - idx;
            } else if idx < 0{
                //println!("{idx}");
                idx = 100 + idx
            }
            v[idx as usize] += 1;
            //println!("{} - {}", v[idx as usize],idx);
        }
    }
    //println!("{:?}", v);
    let max_element = v.iter()
    .enumerate()
    .max_by(|(idx_a, a), (idx_b, b)| {
        a.partial_cmp(b)
            .unwrap()
            .then_with(|| idx_b.cmp(idx_a)) 
    });

    if let Some((index, value)) = max_element {
        //println!("Max value is {} at index {}", value, index);
        return (index as i32 + 1)*value;
    } else{
        return 1;
    } 

}