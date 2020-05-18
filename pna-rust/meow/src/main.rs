#[macro_use]
extern crate clap;
use clap::App;

// fn main () {
//     App::new ("myapp")
//         .version("1.0")
//         .about("Does great things!")
//         .author("Kevin K.")
//         .get_matches();
// }

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // match m.value_of("argumnen1").unwrap() {
    //     //
    // }
}