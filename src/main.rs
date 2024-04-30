use std::{cmp::Ordering, io::stdin};
use rand::{thread_rng, Rng};
use colored::Colorize;


fn main() {
    println!("Welcome to the Game! \n Type quit to exit the Game \n");

    let mut score = 0;

    let secretnum = thread_rng().gen_range(1..=100);
    loop {
         
        println!("Please Guess a number Between 1 to 100 !");

        let guess = take_input();

        if guess.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        let guess:u32 = parse_input(guess);

        if guess == 101 {
            continue;
        }

        let result = compare_val(guess, secretnum, score);

        if result {
            break
        }
        println!("\n=================================\n");
        score = score + 1;
    }
}


fn take_input() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("ERROR: While Taking Input!");
    guess
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

fn parse_input(guess:String) -> u32 {
    let guess:u32 = match guess.trim().parse() {
        Ok(val) => {
            if (val <= 100) & (val >= 1) {
                val
            } else {
                101
            }
        },
        Err(_) => {
            101
        },
    };
    guess
}

