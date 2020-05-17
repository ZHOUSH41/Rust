use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // 当其值是 Err 时，该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
    // 第十三章 会更详细的介绍闭包。现在你需要理解的是 unwrap_or_else 会将 Err 的内部值，
    // 也就是示例 12-9 中增加的 not enough arguments 静态字符串的情况，传递给闭包中位于两道竖线间的参数 err

    // env::args返回的是一个迭代器，所以是把迭代器的所有权给了 new
    let config = Config::new(env::args()).unwrap_or_else( {
        |err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let config = Config::new(&args).unwrap_or_else(
//         |err| {
//             eprintln!("Problem parsing arguments: {}", err);
//             process::exit(1);
//         }
//     );
//
//     //println!("Searching for {}", config.query);
//     //println!("In file {}", config.filename);
//
//     if let Err(e) = minigrep::run(config) {
//         println!("Apllication error: {}", e);
//         process::exit(1);
//     }
//
//
//     // let contents = fs::read_to_string(filename)
//     //     .expect("Something went wrong reading the file");
//
//     //println!("with text:\n{}", contents);
// }
