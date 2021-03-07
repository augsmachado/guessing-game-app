use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    // Generating a random number    
    // This function is optimised for the case that only a single sample is made from the given range
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);
    
    //println!("The secret number is: {}", secret_number);

    // Attempts counter
    let mut attempt = 0;

    // Allowing multiple guesses with looping
    loop {
        // Processing a guess and storing a value with variable
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Casting to convert String to real number and handling a invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        // Comparing the guess to the secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won after {} attempts!", attempt);
                break; // Quiting after the correct answer
            },
        }

        // Increment 1 after each attempt
        attempt = attempt + 1;
    }
    
}