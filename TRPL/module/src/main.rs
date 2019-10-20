mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breath_in();
        }
    }
    fn breath_in() {
        println!("Breath in.");
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod performance_group {
    use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
    }
}

fn main() {
    //crate::sound::instrument::clarinet();

    //sound::instrument::clarinet();
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
    //不能编译
    //println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    performance_group::clarinet_trio();
}
