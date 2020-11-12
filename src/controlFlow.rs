fn if_else(n: u32){
    if (n % 2) == 0 {
        println!("even");
    }else{
        println!("odd");
    }
}

fn if_else_if_else(n: u32){
    if n == 0 {
        println!("zero");
    }else if (n % 2) == 0{
        println!("even");
    }else {
        println!("odd");
    }
}

fn if_else_var(go: bool){
    let k = if go {
        10
    } else {
        20
    };
    println!("{}", k);
}
fn loops(n: u32) {
    let mut i = 0;
    loop {
        println!("Hello rust!");
        i += 1;
        if i >= n {
            break;
        }
    }
}

fn loop_abc() {
    let mut i = 0;
     loop {
        println!("Hello rust!");
    }
}

fn while_loop(n: u32) {
    let mut i = 0;
    while i < n {
        println!("Hello rust {}", i);
        i += 1;
    }
}

fn for_loop_vec() {
    let a = vec![10, 20, 30];

    for i in a {
        println!("{}", i);
    }
}

fn for_loop_var(n: u32) {
    for i in 1..n {
        println!("A {}", i);
    }

    for i in 1..=n {
        println!("B {}", i);
    }
}

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