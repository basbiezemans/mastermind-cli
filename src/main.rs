use clap::Parser;
use log::error;
use mastermind_cli::code;
use mastermind_cli::user_input::get_input;

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
        Ok(input) => {
            let code = code::from_string(input);
            match code {
                Some(guess) => println!("You guessed {}", guess),
                None => println!(
                    "Please enter 4 digits, \
                    where each digit is between 1 and 6, e.g. 1234"
                ),
            }
        }
        Err(error) => error!("Input error: {}", error),
    }
}
