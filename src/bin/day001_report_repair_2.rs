fn main() {
    let mut vec1 = Vec::new();
    
    println!("Input");
    while let Some(input) = single_input() { 
        vec1.push(input);
    }
}

fn single_input() -> Option<i32> {
    let mut s = String::new();
    let input = std::io::stdin().read_line(&mut s).expect("err read");
    if input == 0 {
        return None;
    } else {
        return Some(s.trim().parse::<i32>().unwrap());
    }
}