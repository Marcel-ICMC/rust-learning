use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{Error, ErrorKind};


struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
           return Err(Error::new(ErrorKind::Other, "oh no!"));
        }

        Ok(Guess {value})
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(n) => n,
                Err(_) => {
                    println!("Please type a number between 1 and 100.");
                    continue
                },
            },
            Err(_) => {
                println!("Please type a number!");
                continue
            },
        };

        println!("You guessed: {}", guess.value());

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