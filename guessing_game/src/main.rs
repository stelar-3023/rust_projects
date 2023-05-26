use std::io; // bring io library into scope

fn main() { // main function
    println!("Guess the number!"); // print line

    println!("Please input your guess.");

    let mut guess = String::new(); // create a mutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
