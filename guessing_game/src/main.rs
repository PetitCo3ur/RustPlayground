use std::io;
use std::cmp::Ordering;

use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number : i32 = rand::thread_rng().gen_range(1..=100);
        
    loop {
        let mut guess : String = String::new();
        println!("Input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
             },
        }
    }
}
