fn main() {
    // const MAX_POINTS: u32 = 100_000;
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    another_function(5);
    let x = five();
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was fasle");
    }
}

fn another_function(x: i32) {
    println!("The value of x is : {}", x);
}

fn five() -> i32 {
    5
}
