use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // This line creates a mutable variable that is currently bound to new, empty instance of a String.
    let mut guess = String::new(); // mut means mutable

    io::stdin()
        .read_line(&mut guess) // & means a reference, and references are immutable by default
        .expect("Failed to read line.");
    
    // this guess is shadowing the guess variable on line 15, so I don't have to create guess_number
    // this feature is often used when convert.
    // u32 is 32-bit number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
