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
