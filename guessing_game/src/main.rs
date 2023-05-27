use rand::Rng; // bring Rng trait into scope
use std::cmp::Ordering;
use std::io; // bring io library into scope // bring Ordering enum into scope

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

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
