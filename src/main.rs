use std::cmp::Ordering;
use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Welcome to the guessing game!!!");
    
    let winner_number:u32 = thread_rng().gen_range(1..101);
    
    loop {
        let mut guess = String::new();
        
        println!("Try to guess a number");

        io::stdin().read_line(&mut guess).expect("Impossible to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&winner_number) {
            Ordering::Greater => println!("Too Big..."),
            Ordering::Less => println!("Too Low..."),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

    println!("You reached the target number, congratulations!!!");
    println!("The correct answear is: {}", winner_number);
}