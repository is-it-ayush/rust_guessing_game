use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Rust is ahead of time compiled and then run. On Windows, It creates a exe otherwise on all other platforms it's just a file.

fn main() {
    // This is a cool banner. I like when my programs look nice.
    println!("------------------------------------------");
    println!("Rusty Guessing Game!");
    println!("Could you do it under 5 tries? *EVIL LAUGHS*");
    println!("------------------------------------------");

    // Game Logic
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Making the game more fun by adding a counter.
    // The counter will keep track of the number of guesses.
    let mut counter = 0;

    // Loops are a way to loop indefinitely over instructions unless if break; is used to break out of the loop.
    loop {
        //  Here we ask the user for an input
        println!("Please input your guess.");

        /*
         * We create a mutable variable. Variables in Rust are immutable by default.
         * We need to explicitly define a variable mutable if we want to change it during runtime.
         * We do this by giving it a keyword 'mut'.
         */
        let mut guess = String::new();

        // Reading for the user input.
        // Note: Look how Rust forces us to except for an error since read_line returns a result type, a enum with two variants.
        // Ok(T) -> Which means, everything is good and returns the T Type value.
        // Err(E) -> Something went wrong and the program has crashed with the error E. This error needs to be managed.
        io::stdin()
            .read_line(&mut guess)
            .expect("There was an error while reading the input.");

        /*
         * Now we are converting the string input to a unsigned 32 bit integer. The same type as our secret number.
         * This also returns a result as described above. The error needs to be handled properly for program to exit out.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Using a wildcard as a catch all. No matter what we get. Just continue.
        };

        /*
         * Rust has pattern matching. It is nothing but a, "Hey look this can variable can be like this AND this." (notice AND).
         * Every case needs to be managed explicitly or by using wildcards `_`. (catch all)
         * Here we use break; to break out of the loop. This is important otherwise it'll run indefinitely.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                match Option::Some(counter) {
                    Some(x) if x <= 5 => {
                        println!("It took you {} tries. You're a genius", counter);
                        break;
                    }
                    Some(x) if x <= 10 => {
                        println!("It took you {} tries. Not Bad!", counter);
                        break;
                    }
                    Some(x) if x <= 20 => {
                        println!(
                            "It took you {} tries. It's okay. You'll do better next time!",
                            counter
                        );
                        break;
                    }
                    _ => panic!(),
                }
            }
            Ordering::Greater => println!("{}", "Too Big!".red()),
        }

        counter += 1;
    }
}
