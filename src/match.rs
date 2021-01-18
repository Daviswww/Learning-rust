fn matchs(n: u32) {
    match n {
        1 => println!("1"),
        | 2 | 3 | 4 => println!("234"),
        5..=6 => println!("5~6"),
        _ => println!("GG"),
    }
}

fn match_tuple() {
    let tuple = (1, 1);

    match tuple {
        (0, x) => println!("(0, {})", x),
        (x, y) if x == y => println!("({}, {})", x, y),
        _ => println!("GG"),
    }
}

fn match_v(n: u32) {
    match n {
        k @ 1 ..= 3 => println!("Value {}", k),
        k @ 4 ..= 6 => println!("Value {}", k),
        _ => println!("GG"),
    }
}