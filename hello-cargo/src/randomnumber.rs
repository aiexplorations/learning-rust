use std::io;
use rand::Rng;


fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_ng().get_range(1, 101);

    println!("The secret num is {}", secret_number);

    println!("Please input a guess:");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);

}