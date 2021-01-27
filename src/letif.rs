fn iflet() {
    let s = Some("ok");

    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }

    if let Some(i) = Some("ok") {
        println!("{}", i);
    }else {
        println!("nall");
    }
}

fn whilelet() {
    let mut s = Some(0);
    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 1);
        }
    }
}
fn main(){
    whilelet();
}