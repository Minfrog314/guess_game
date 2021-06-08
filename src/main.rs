use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Some random guessing game. Hooray");

    let scrt_num = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", scrt_num);

    loop {

        println!("Input your guess here: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&scrt_num) {
            Ordering::Less => println!("That number happens to be too small."),
            Ordering::Greater => println!("That number is in fact too big."),
            Ordering::Equal => {
                println!("You win. Yay.");
                break;
            }
        }
    }
}
