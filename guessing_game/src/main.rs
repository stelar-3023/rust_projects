use rand::Rng; // bring Rng trait into scope
use std::io; // bring io library into scope

fn main() {
    // main function
    println!("Guess the number!"); // print line

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate random number

    println!("The secret number is: {secret_number}",); // print secret number

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
