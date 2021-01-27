use std::collections::HashMap;

fn main(){
    let mut hm: HashMap<String, i32> = HashMap::new();

    hm.insert(String::from("Key"), 123);
    hm.insert(String::from("Key1"), 456);

    for(k, v) in &hm {
        println!("{} {}", k, v);
    }

    match hm.get(&String::from("Key")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match!"),
    }

    hm.remove(&String::from("Key1"));
}