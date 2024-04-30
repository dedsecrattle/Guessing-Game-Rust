use std::io::stdin;

pub fn take_input() -> String {
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("ERROR: While Taking Input!");
    guess
}

pub fn parse_input(guess:String) -> u32 {
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