struct Object{
    width: u32,
    height: u32,
}

fn area(obj: Object) -> u32 {
    return obj.width * obj.height;
}

pub fn run() {
    let o = Object {
        width: 10,
        height: 5,
    };

    println!("{}", area(o));
}
