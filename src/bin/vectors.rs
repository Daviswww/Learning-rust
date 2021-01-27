fn base(){
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);   

    for i in &v {
        println!("{}", i);
    }
    v.push(10);

    println!("{:?} {} {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop())
}
#[derive(Debug)]
enum Point{
    Float(f64),
    Int(i32),
    Text(String),
}

fn enum_vec() {
    let ev = vec![
        Point::Float(3.14), 
        Point::Int(123), 
        Point::Text(String::from("Hello")),
    ];
    println!("{:?}", &ev);
}
fn main(){
    enum_vec();
}