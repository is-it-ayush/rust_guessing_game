use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Rust is ahead of time compiled and then run. On Windows, It creates a exe otherwise on all other platforms it's just a file.

fn main() {
    // This is a cool banner. I like when my programs look nice.
    println!("------------------------------------------");
    println!("Welcome to the Guessing Game!");
    println!("------------------------------------------");

    // Game Logic
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        //  Here we ask the user for an input
        println!("Please input your guess.");

        /*
         * We create a mutable variable. Variables in Rust are immutable by default.
         * We need to explicitly define a variable mutable if we want to change it during runtime.
         * We do this by giving it a keyword 'mut'.
         */
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("There was an error while reading the input.");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("There was a an error parsing the input to u32.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
