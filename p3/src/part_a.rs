pub fn part_a(lines: String) -> (u128, String){
    
    
    let mut maxscore:u128 = 0;
    let mut maxstring:&str = "a";
    for line in lines.lines(){
        let len = line.len() as u128;
        //println!("{len}");
        let mut v:Vec<u128> = Vec::new();
        let mut score:u128 = 0;
        for l in line.chars(){
            if !v.contains(&1) && l.is_lowercase(){
                score += 1;
                v.push(1);
            }  else if !v.contains(&2) && l.is_uppercase(){
                score += 1;
                v.push(2);
            } else if !v.contains(&3) && l.is_ascii_digit(){
                score += 1;
                v.push(3);
            } else {
                continue;
            }
        }
        // println!("{score} {len}");
        score = score * len ;
        if score >= maxscore {
            maxscore = score;
            maxstring = line;
        }
    }
    (maxscore, maxstring.to_string())
}