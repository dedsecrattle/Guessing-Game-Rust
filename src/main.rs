mod parser;

use std::{cmp::Ordering};
use rand::{thread_rng, Rng};
use colored::Colorize;


fn main() {
    println!("Welcome to the Game! \n Type quit to exit the Game \n");

    let mut score = 0;

    let secret_num = thread_rng().gen_range(1..=100);
    loop {
         
        println!("Please Guess a number Between 1 to 100 !");

        let guess = parser::take_input();

        if guess.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        let guess:u32 = parser::parse_input(guess);

        if guess == 101 {
            continue;
        }

        let result = compare_val(guess, secret_num, score);

        if result {
            break
        }
        println!("\n=================================\n");
        score = score + 1;
    }
}




fn compare_val(guess:u32, actual:u32, score:i32) -> bool {
    match guess.cmp(&actual) {
        Ordering::Less => {
            println!("{}","Your Guess is too Less!".red());
            false
        },
        Ordering::Equal => {
            println!("{} {}", "You Won , Your Score is".green(),score);
            true
        },
        Ordering::Greater=> {
            println!("{}","Your Guess is too High!".red());
            false
        }
    }
}



