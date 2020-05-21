use serde::{Serialize, Deserialize};
use std::fs::File;
use serde_json;
use std::io::{Write, Read};


#[derive(Serialize, Deserialize, Debug)]
 struct Move {
     step: u32,
     direction: String,
 }

fn main() {
    let a = Move{ step: 100,direction: String::from("left")};
    let serialize = serde_json::to_string(&a).unwrap();
    println!("serialize = {:?}", a);
    let mut file = File::create("output.json").expect("create failed!");
    file.write_all(serialize.as_bytes()).expect("write failed!");

    let mut file = File::open("output.json").expect("cannot open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read");
    let deserialize: Move = serde_json::from_str(&contents).unwrap();
    println!("deserialize = {:?}", deserialize);
}
