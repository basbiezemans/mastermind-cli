use clap::Parser;
use mastermind_cli::{
    code::random as make_secret, feedback, show, user_input::read_line,
};
use pluralizer::pluralize;

#[derive(Parser, Debug)]
#[clap(
    about = "Mastermind, the code-breaking game",
    after_help = "Example: mastermind-cli --turns 12"
)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    turns: u8,
}

fn main() {
    let args = Args::parse();
    let limit = args.turns;
    let secret = make_secret();

    println!("+------------------------------------+");
    println!("| Mastermind, the code-breaking game |");
    println!("+------------------------------------+");
    println!("You have {limit} turns to guess the code. Good luck!");

    for turns_left in (0..limit).rev() {
        println!(
            "\nYou have {: >2} {} left.",
            (1 + turns_left),
            pluralize("turn", (1 + turns_left).into(), false)
        );

        let guess = read_line("Guess: ").unwrap_or_default();

        if guess == secret {
            let turns_taken = limit - turns_left;
            println!(
                "\nYou won in {}!\n",
                pluralize("guess", turns_taken.into(), true)
            );
            break;
        } else if turns_left == 0 {
            println!("No such luck. The secret was {secret}\n");
        } else {
            println!("Hint: {}", show(feedback(guess, secret.clone())));
        }
    }
}
