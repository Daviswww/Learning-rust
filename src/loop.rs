
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