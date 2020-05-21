use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::Deserializer;
use ron::de::from_str;
use std::str::from_utf8;
//use std::fs::File;
//use serde_json;
use ron;
//use std::io::{Write, Read};


#[derive(Serialize, Deserialize, Debug)]
 struct Move {
     step: u32,
     direction: String,
 }

fn main() {
    /// write with "serde_json" crate
    // let a = Move{ step: 100,direction: String::from("left")};
    // let serialize = serde_json::to_string(&a).unwrap();
    // println!("serialize = {:?}", a);
    // let mut file = File::create("output.json").expect("create failed!");
    // file.write_all(serialize.as_bytes()).expect("write failed!");
    //
    // let mut file = File::open("output.json").expect("cannot open file!");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("cannot read");
    // let deserialize: Move = serde_json::from_str(&contents).unwrap();
    // println!("deserialize = {:?}", deserialize);

    /// write with ron crate
    let a = Move {step: 100, direction: String::from("left")};
    let pretty = PrettyConfig{
        depth_limit: 0,
        new_line: "".to_string(),
        indentor: "".to_string(),
        separate_tuple_members: false,
        enumerate_arrays: false
    };
    let mut s = to_string_pretty(&a, pretty).expect("Serialize failed");
    println!("{:?}", &s);

    //let des= s.as_bytes();
    let b:Move = from_str(&s).unwrap();
    println!("{:?}", &b);
}
// use ron::de::from_str;
// use serde::Deserialize;
// use std::collections::HashMap;
//
// #[derive(Debug, Deserialize)]
// struct Config {
//     boolean: bool,
//     float: f32,
//     map: HashMap<u8, char>,
//     nested: Nested,
//     option: Option<String>,
//     tuple: (u32, u32),
// }
//
// #[derive(Debug, Deserialize)]
// struct Nested {
//     a: String,
//     b: char,
// }
//
// const CONFIG: &str = "
// /*
//  * RON now has multi-line (C-style) block comments!
//  * They can be freely nested:
//  * /* This is a nested comment */
//  * If you just want a single-line comment,
//  * do it like here:
// // Just put two slashes before the comment and the rest of the line
// // can be used freely!
// */
// // Note that block comments can not be started in a line comment
// // (Putting a /* here will have no effect)
// (
//     boolean: true,
//     float: 8.2,
//     map: {
//         1: '1',
//         2: '4',
//         3: '9',
//         4: '1',
//         5: '2',
//         6: '3',
//     },
//     nested: Nested(
//         a: \"Decode me!\",
//         b: 'z',
//     ),
//     option: Some(\t  \"Weird formatting!\" \n\n ),
//     tuple: (3 /*(2 + 1)*/, 7 /*(2 * 5 - 3)*/),
// )";
//
// fn main() {
//     let config: Config = match from_str(CONFIG) {
//         Ok(x) => x,
//         Err(e) => {
//             println!("Failed to load config: {}", e);
//
//             std::process::exit(1);
//         }
//     };
//
//     println!("Config: {:?}", &config);
// }