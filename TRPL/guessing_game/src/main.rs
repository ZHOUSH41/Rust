use rand::Rng;
use std::cmp::Ordering;
use std::io; //显式地引用类型

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is : {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // guess是一个可变的变量

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error input, please type a number");
                continue;
            }
        };

        println!("You guessed: {}", &guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
