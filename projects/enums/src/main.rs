#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddKind,
    address: String,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };

    println!("Hello, world!");
}
