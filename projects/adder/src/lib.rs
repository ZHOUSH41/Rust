pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
// #[derive(Debug)]
// pub struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }

//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn larger_can_holder_smaller() {
//         let larger = Rectangle {
//             length: 8,
//             width: 7,
//         };
//         let smaller = Rectangle {
//             length: 5,
//             width: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             length: 8,
//             width: 7,
//         };
//         let smaller = Rectangle {
//             length: 5,
//             width: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }
