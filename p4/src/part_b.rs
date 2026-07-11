use std::collections::HashMap;

pub fn swap_branches(text: String) -> i32 {
    let mut h: HashMap<String, String> = HashMap::new();
    h.insert(r"|-o".to_string(), r"LR".to_string());
    h.insert(r"o-|".to_string(), r"LL".to_string());
    h.insert(r"\|/".to_string(), r"T".to_string());
    h.insert(r"-@-".to_string(), r"C".to_string());
    h.insert(r"/|\".to_string(), r"D".to_string());
    h.insert(r"|".to_string(), r"S".to_string());
    h.insert(r"#####".to_string(), r"E".to_string());

    let mut ans = 0;
    
    let mut v: Vec<String> = text.lines().map(|c| c.to_string()).collect();
    v.reverse(); 
    let vclone = v.clone();

    let mut i = 1;
    let mut j = 0;

    loop {
        while i < v.len() {
            let u_str = v.get(i).unwrap().trim();
            let v_str = v.get(j).unwrap().trim();

            let h1 = h.get(u_str).cloned().unwrap_or_else(|| "".to_string());
            let h2 = h.get(v_str).cloned().unwrap_or_else(|| "".to_string());

            // If we hit the flower head elements, we are done climbing
            if h1 == "T" || h1 == "C" || h1 == "D" {
                break;
            }

            // Skip over empty stem lines ("S") or unrelated lines so they don't break our window
            if h1 != "LL" && h1 != "LR" {
                i += 1;
                continue;
            }
            if h2 != "LL" && h2 != "LR" {
                j = i; // Move j up to look for a valid starting leaf
                i += 1;
                continue;
            }

            // println!("{:?} {:?}", h1, h2);

            if (h1 == "LL" && h2 == "LR") || (h1 == "LR" && h2 == "LL") {
                ans += 1;
                j = i; 
                i += 1;
            } else {
                j = i; 
                i += 1;
            }
        }

        if i >= v.len() {
            break;
        }
        
        let u_str = v.get(i).unwrap().trim();
        let h1 = h.get(u_str).cloned().unwrap_or_else(|| "".to_string());
        if h1 == "T" || h1 == "C" || h1 == "D" {
            break;
        }
        i += 1;
        j += 1;
    }

    ans
}