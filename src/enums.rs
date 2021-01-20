#![allow(dead_code)]
mod input;

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self{
            Direction::Up(_) => Keys::UpKey(String::from("W")),
            Direction::Down(_) => Keys::DownKey(String::from("s")),
            Direction::Left(_) => Keys::LeftKey(String::from("a")),
            Direction::Right(_) => Keys::RightKey(String::from("d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self{
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    // input::single_input_string_I32();

    let u = Direction::Up(Point {x: 0, y: 1});
    let k = u.match_direction();

    println!("{:?}", k);
}
