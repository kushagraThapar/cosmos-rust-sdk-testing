use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guessed number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
