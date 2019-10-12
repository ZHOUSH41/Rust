use std::collections::HashMap;

fn main() {
    //let v: Vec<i32> = Vec::new();
    //let m = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yeloow"), 50);
}
