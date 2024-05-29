use rand::Rng;
use std::num::ParseIntError;

const MAX_GUESS: i32 = 100;
const CHANCES: i32 = 5;

fn get_user_input() -> Result<i32, ParseIntError> {
    let mut user_input_str = String::new();

    std::io::stdin()
        .read_line(&mut user_input_str)
        .expect("can't get a user input");

    user_input_str.trim().parse()
}

pub fn main() {
    println!("Guess Number from 0 to {}:", MAX_GUESS);
    println!("You Have {} Chance to guess the correct number", CHANCES);

    let number = rand::thread_rng().gen_range(0..MAX_GUESS);
    let mut chance = CHANCES;

    while chance > 0 {
        match get_user_input() {
            Ok(user_guess) => {
                if user_guess > number {
                    println!("Your guess is greater then number");
                } else if user_guess < number {
                    println!("Your guess is less then number");
                } else {
                    println!("Correct 🎉");
                    return;
                }
                chance -= 1;
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        };
    }

    if chance == 0 {
        println!("\nGAME OVER");
        println!("Correct number is {}", number);
    }
}
