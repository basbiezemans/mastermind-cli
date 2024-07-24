use clap::Parser;
use mastermind_cli::{feedback, is_valid, make_secret, show, user_input::read_line};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    turns: u8,
}

fn main() {
    let args = Args::parse();
    let limit: u8 = args.turns;
    let secret: String = make_secret();

    println!("+------------------------------------+");
    println!("| Mastermind, the code-breaking game |");
    println!("+------------------------------------+");
    println!("You have {} turns to guess the code. Good luck!", limit);

    for k in (1..=limit).rev() {
        println!(
            "\nYou have {: >2} turn{} left.",
            k,
            if k > 1 { "s" } else { "" }
        );

        let input: String = read_line("Guess: ").unwrap();
        let guess: String = input.clone();

        let game_over = k == 1;

        if !is_valid(input) {
            println!(
                "Please enter 4 digits, \
                where each digit is between 1 and 6, e.g. 1234"
            );
            continue;
        }

        if guess == secret {
            println!("You won!");
            break;
        } else if game_over {
            println!("No such luck. The secret was {}", secret);
        } else {
            println!("Hint: {}", show(feedback(guess, secret.clone())));
        }
    }
}
