use std::cmp::Ordering; // Used for comparing the guess with the secret number
use std::io; // Used for input/output operations
use rand::RngExt; // Used for generating random numbers

fn main() {
    println!("Guess the number!"); // Prints the message "Guess the number!" to the console

    let secret_number: u32 = rand::rng().random_range(1..=100); // Generates a random number between 1 and 100 (inclusive)

    // println!("The secret number is: {}", secret_number); // Prints the secret number to the console
    let mut guesses: u32 = 0; // Initializes the number of guesses to 0

    loop { // Loops until the user guesses the number
        guesses += 1; // Increments the number of guesses by 1
        println!("Please input your guess."); // Prompts the user to enter a guess

        let mut guess = String::new(); // Creates a new mutable String to store the guess

        io::stdin() // Reads the input from the user
            .read_line(&mut guess) // Reads the input from the user and stores it in the guess variable
        .expect("Failed to read line"); // Exits the program if the input is not a valid line

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // Trims whitespace and converts the guess to a u32

        println!("You guessed: {}", guess); // Prints the guess to the console

        match guess.cmp(&secret_number) { // Compares the guess with the secret number
            Ordering::Less => println!("Too small!"), // If the guess is less than the secret number, print "Too small!"
            Ordering::Greater => println!("Too big!"), // If the guess is greater than the secret number, print "Too big!"
            Ordering::Equal => { // If the guess is equal to the secret number, print "You win!" and exit the loop
                println!("You win!");
                println!("Guesses took to win: {}", guesses);
                break;
            }
        }

    }
}