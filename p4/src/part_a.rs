use std::collections::HashMap;
pub fn replace_funny_chars(text:String, n:i32) -> i32{
    let mut h:HashMap<String, String> = HashMap::new();
    h.insert(r"|-o".to_string(), r"L".to_string());
    // h.insert(r"|-o".to_string(), r"L".to_string());
    h.insert(r"o-|".to_string(), r"L".to_string());
    h.insert(r"\|/".to_string(), r"T".to_string());
    h.insert(r"-@-".to_string(), r"C".to_string());
    h.insert(r"/|\".to_string(), r"D".to_string());
    h.insert(r"|".to_string(), r"S".to_string());
    h.insert(r"#####".to_string(), r"E".to_string());
    let mut count = 0;
    let mut ans = 0;
    for t in text.lines().rev(){
        // to find L give we cut L or S
        //println!("{:?}",h.get(t.trim()).unwrap());
        let v = h.get(t.trim()).unwrap();
        if v == "L" ||   v == "S"{
            if count <= n{
                count += 1;
            }
            if count > n && v == "L"{
                ans += 1;
            }
        }
    }

    ans


}