pub fn run(){
    println!("Num: {}, {}", 1, 2);

    println!("{0} is {0} and {1} haha {2}.", "Bob", "Alice", "John");

    println!("{a} is {b} and {c} haha {a}.", b = "Bob", a = "Alice", c = "John");

    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}.", 10, 10, 10);
}