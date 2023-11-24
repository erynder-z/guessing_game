use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Create a new mutable variable `guess` of type `String`

        io::stdin()
            .read_line(&mut guess) // Read input from the user and store it in `guess`
            .expect("Failed to read line"); // Crash the program if an error occurs

        let guess: u32 = match guess.trim().parse() {
            // Parse the input string into an unsigned 32-bit integer
            Ok(num) => num, // If parsing is successful, assign the parsed value to `guess`
            Err(_) => continue, // If parsing fails, skip the current iteration of the loop
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            // Compare the value of `guess` with `secret_number`
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
