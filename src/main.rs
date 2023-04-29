use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    
    println!("Welcome to the guessing game!");
    println!("Choose a dificult level:\n");
    println!("(E) - Easy\n(M) - Medium\n(H) - Hard\n");

    let mut limit_chances:String = String::new();

    io::stdin().read_line(&mut limit_chances).expect("Fail to readline");

    let number_of_chances:u8;

    if limit_chances.trim().to_lowercase() == "e" {
        number_of_chances = 15;
    }
    else if limit_chances.trim().to_lowercase() == "m" {
        number_of_chances = 10;
    }
    else {
        number_of_chances = 5;
    }

    println!("You have {} chances to guess the winner number\n", number_of_chances);
    
    println!("Good Luck!");

    let secret_number:u8 = rand::thread_rng().gen_range(1..101);
    
    let mut chances_taken:u8 = 1;

    loop {
        println!("\nChance number: #{}", chances_taken);
        println!("Throw a number...");

        let mut guess:String = String::new();


        io::stdin().read_line(&mut guess).expect("Fail to read line");

        let guess:u8 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue 
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too big...");
                chances_taken+=1;
            },
            Ordering::Less => {
                println!("Too low...");
                chances_taken+=1;
            },
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }

        if chances_taken == number_of_chances {
            println!("Sorry you are out of chances");
            break;
        }
    }
}