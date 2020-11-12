struct Object{
    width: u32,
    height: u32,
}

impl Object {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    
    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
        }
    }
}
// 分開寫法
// impl Object {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
// }

// impl Object {
//     fn new(width: u32, height: u32) -> Object {
//         Object {
//             width,
//             height,
//         }
//     }
// }

pub fn run() {
    let obj = Object::new(10, 3);

    println!("{}", obj.area());
}
