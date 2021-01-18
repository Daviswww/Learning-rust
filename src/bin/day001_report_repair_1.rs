fn main() {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    
    println!("Input");
    while let Some(input) = single_input() { 
        vec1.push(input);
        vec2.push(2020-input);
    }

    vec1.sort();
    vec2.sort();
    let mut top1 = vec1.pop().unwrap();
    let mut top2 = vec2.pop().unwrap();
    loop{
        if top1 == top2 {
            println!("{}", top1 * (2020-top1));
            break;
        }
        if top1 < top2 {
            top2 = vec2.pop().unwrap();
        }else{
            top1 = vec1.pop().unwrap();
        }
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