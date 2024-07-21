use clap::Parser;
use log::error;
use user_input::get_input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    turns: u8,
}

fn main() {
    let args = Args::parse();
    let limit: u8 = args.turns;
    println!("+------------------------------------+");
    println!("| Mastermind, the code-breaking game |");
    println!("+------------------------------------+");
    println!("You have {} turns to guess the code. Good luck!", limit);

    match get_input("Guess: ") {
        Ok(guess) => println!("You guessed {}", guess),
        Err(error) => error!("Input error: {}", error),
    }
}

mod user_input {
    use std::io::stdin;
    use std::io::Write;
    pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Err(e) => Err(e),
            Ok(_) => Ok(input.trim().to_string()),
        }
    }
}
