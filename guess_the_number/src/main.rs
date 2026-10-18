use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Gets a secret number between 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Guess tracker
    let mut num_guess: u32 = 0; 

    loop { // while True:
        if num_guess < 10 {
            println!("Please input your guess.");

            let mut guess = String::new(); // New string

            io::stdin()
                .read_line(&mut guess) // puts input into string reference to a mut named guess
                .expect("Failed to read line"); // Error handling

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num, // returns num if all good.
                Err(_) => continue, // _: match all errors, ignores them, goes to next iteration
            };

            println!("You guessed: {guess}");
            // Guess counter
            num_guess += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too low! Try again."),
                Ordering::Greater => println!("Too high! Try again."),
                Ordering::Equal => {
                    println!("You win! Good Job!");
                    println!("You guessed the number in {num_guess} guesses.");
                    break;
                }
            } 
            
        } else {

            println!("You have exceeded the maximum number of guesses. The secret number was: {secret_number}");
            break;

        }
    } 
}