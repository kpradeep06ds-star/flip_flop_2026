pub fn part_b(text:String) -> i32 {
    let mut diffvalue;
    let mut total_diff = 0;
    for i in text.lines(){
        
        diffvalue = 60 as i32 - i.trim().parse::<i32>().unwrap();
        //println!("{diffvalue}");
        if diffvalue > 0 {
            total_diff += diffvalue;
        } else {
            total_diff += (-1)*diffvalue*5;
        }
    }
    total_diff
}