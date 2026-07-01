use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! ");

    let secret_number = rand::random_range(1..=100);

    let mut guess_input = String::new();
    loop {
        println!("Please input your guess : ");
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line !");

        println!("You guessed : {guess_input}");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => {
                guess_input.clear();
                num
            }
            Err(err) => {
                println!("{err} : {guess_input}");
                guess_input.clear();
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
    }
}
