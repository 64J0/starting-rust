// Syntax to import a library into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // let creates a variable
        // in rust variables are immutable by default
        // mut before the variable name makes it mutable
        let mut guess = String::new();  // returns a new instance of a string
                                        // :: syntax indicates that new is an
                                        // associated function of the String type.
                                        // Some lang call this a static method.

        io::stdin()
            .read_line(&mut guess) // &mut means the reference for the mutable guess value
            .expect("Failed to read line.");

        // shadow the previous guess value to change its type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
