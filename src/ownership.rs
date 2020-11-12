pub fn run(){
    let mut v = Vec::new();

    for i in 1..1000{
        v.push(i);
    }

    take(v);
    println!("finish!");

    add(1, 2);
}

fn take(v: Vec<i32>){
    println!("{} {}", v[10], v[100]);
    println!("Take: {}", v[10] + v[100]);
}

pub fn add(a: i32, b: i32){
    println!("Add a + b = {}", a + b);
}
