fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    assert_eq!(result, 20);
    //println!("Hello, world!");

    println!("50 fib is {}", fib(50));
}

fn fib(n: i32) -> i64 {
    let mut a = 0;
    let mut b = 1;
    let mut count = 1;
    while count < n {
        let temp = b;
        b = a + b;
        a = temp;
        count += 1;
    }
    return b;
}
