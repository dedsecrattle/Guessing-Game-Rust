use std::{cmp::Ordering, io::stdin};
use rand::{thread_rng, Rng};
use colored::Colorize;


fn main() {
    println!("Welcome to the Game! \n Type quit to exit the Game \n");

    let mut score = 0;

    let secretnum = thread_rng().gen_range(1..=100);
    loop {
         
        println!("Please Guess a number Between 1 to 100 !");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("ERROR: While Taking Input!");

        if guess.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        let guess:u32 = match guess.trim().parse() {
            Ok(val) => {
                if (val <= 100) & (val >= 1) {
                    val
                } else {
                    continue;
                }
            },
            Err(_) => {
                continue;
            },
        };

        match guess.cmp(&secretnum) {
            Ordering::Less => println!("{}","Your Guess is too Less!".red()),
            Ordering::Equal => {
                println!("{} {}", "You Won , Your Score is".green(),score);
                break;
            },
            Ordering::Greater=> println!("{}","Your Guess is tooo High!".red())
        }
        println!("\n=================================\n");
        score = score + 1;
    }
}
