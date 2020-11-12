pub fn run() {
    let name = "Bob";
    println!("My name is {}.", name);

    const ID: i32 = 001;
    println!("My ID is {}.", ID);

    let ( a, b ) = ("Alice", 23);
    println!("{} is {}.", a, b);

    let t = (1, 'a', false);
    println!("{} {} {}", t.0, t.1, t.2);
    
    let k = (3, (2, (1, 'a', false)));
    println!("{:?}", k.1);
    println!("{:#?}", k.1);
}