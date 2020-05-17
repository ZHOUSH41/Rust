use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // 需要clone的版本
        // if args.len() < 3 {
        //     return Err("not enough arguments.");
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();
        // let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query,filename,case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // iterator version
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();
    let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
        );
    }
}
// use std::error::Error;
// use std::fs;
// use std::env;
//
// pub struct Config {
//     pub query: String,
//     pub filename: String,
//     pub case_sensitive: bool,
// }
// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough argments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//
//         Ok(Config{query, filename,case_sensitive})
//     }
// }
//
// pub fn run(conig: Config) -> Result<(), Box<dyn Error>>{
//     let contents = fs::read_to_string(conig.filename)?;
//
//     // for line in search(&conig.query, &contents) {
//     //     println!("{}", line);
//     // }
//     let results = if conig.case_sensitive {
//         search(&conig.query, &contents)
//     } else {
//         search_cast_insensitive(&conig.query, &contents)
//     };
//     for line in results {
//         println!("{}", line);
//     }
//     Ok(())
// }
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }
// pub fn search_cast_insensitive<'a> (
//     query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }
//     results
// }
//