use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut num_guess: u32 = 0; 
    loop {
        if num_guess < 10 {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");
            num_guess += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too low! Try again."),
                Ordering::Greater => println!("Too high! Try again."),
                Ordering::Equal => {
                    println!("You win! Good Job!");
                    break;
                }
            }
        } else {
            println!("You have exceeded the maximum number of guesses. The secret number was: {secret_number}");
            break;
        }
    } 
}