pub fn single_input_string_i32() -> Option<i32> {
    let mut s = String::new();
    let input = std::io::stdin().read_line(&mut s).expect("error read");
    if input == 0 {
        return None;
    } else {
        return Some(s.trim().parse::<i32>().unwrap());
    }
}

pub fn single_input_string() -> Option<String> {
    let mut s = String::new();
    let input = std::io::stdin().read_line(&mut s).expect("error read");
    if input == 0 {
        return None;
    } else {
        return Some(s.trim().parse::<String>().unwrap());
    }
}