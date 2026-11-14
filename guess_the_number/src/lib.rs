// use guess_the_number::Guess;
// TODO: clear out commented old code.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}
impl Guess {
    fn handle_guess(num_guess: u32, guess: u32, secret_number: &u32) -> bool {
        if guess < 1 || guess > 100 {
            println!("Guess value must be between 1 and 100, got {}.", guess);
            return false;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("You win! Good Job!");
                println!("You guessed the number in {num_guess} guesses.");
                return true;
            }
        }
        false
    }
    fn new() -> Guess {
        let guess = rand::thread_rng().gen_range(1..=100);
        Guess { value: guess }
    }
}
pub fn play_game() {
    // Gets a secret number between 1 and 100 inclusive
    let secret_number = Guess::new().value;

    // Guess tracker
    let mut num_guess: u32 = 0;

    loop {
        // while True:
        if num_guess < 10 {
            println!("Please input your guess.");

            let mut guess = String::new(); // New string

            io::stdin()
                .read_line(&mut guess) // puts input into string reference to a mut named guess
                .expect("Failed to read line"); // Error handling

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,     // returns num if all good.
                Err(_) => continue, // _: match all errors, ignores them, goes to next iteration
            };

            println!("You guessed: {guess}");
            // Guess counter
            num_guess += 1;
            if Guess::handle_guess(num_guess, guess, &secret_number) {
                break;
            }
        } else {
            println!("You have exceeded the maximum number of guesses. The secret number was: {secret_number}");
            break;
        }
    }
}
// Tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")]
    fn guess_validity() {
        let mut guess = Guess::new();
        guess.value = 1000;
        assert!(
            guess.value >= 1 && guess.value <= 100,
            "Guess value must be between 1 and 100, got {}.",
            guess.value
        );
    }

    #[test]
    fn guess_correct_event() {
        let secret_number = 50;
        assert_eq!(
            Guess::handle_guess(1, 50, &secret_number),
            true,
            "Problem with guess handler, expected `true`, but got `false` with tested values."
        );
    }
    #[test]
    fn too_many_guesses() {
        let secret_number = 42;
        let num_guess = 10;
        let guess = 30;
        assert_eq!(
            Guess::handle_guess(num_guess, guess, &secret_number),
            false,
            "Guess handler should not return `true` when the number of guesses exceeds the limit."
        );
    }
}
