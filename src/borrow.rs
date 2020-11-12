pub fn run(){
    let mut v = Vec::new();

    for i in 1..1000{
        v.push(i);
    }
    // Return
    v = ret(v);
    println!("{}", v[0]);

    bor(&v);
    println!("{} {}", v[0], v[1]);
}

fn ret(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[10] + v[20]);
    v
}

fn bor(v: &Vec<i32>){
    println!("{}", v[0] + v[1]);
}