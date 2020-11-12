pub fn run() {
    // Array
    let xs : [i32; 5] = [4, 2, 3, 5, 9];
    let xy = &xs[2..4];

    println!("{:?} {:?}", xs, xy);

    // String
    let y = String::from("BobAlice");

    println!("String: {} ", y);
    println!("String: {} ", &y[2..4]);
}
