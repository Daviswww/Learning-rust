fn single_input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("err read");
    return s;
}

fn single_input_type() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("err read");
    return s.trim().parse::<i32>().unwrap();
}