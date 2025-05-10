use clap::Parser;
use mastermind_cli::{
    codemaker::{feedback, secret, show},
    user_input::read_line,
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

enum GameState {
    ExitGame,
    GameOver,
    GameWon(u8),
}

fn main() {
    let args = Args::parse();
    let limit = args.turns;

    loop {
        println!("\n");
        println!("┌────────────────────────────────────┐");
        println!("│ Mastermind, the code-breaking game │");
        println!("└────────────────────────────────────┘");
        println!("\nYou have {limit} turns to break the code. Good luck!");

        let secret = secret();

        match play_game(limit, secret.clone()) {
            GameState::GameOver => {
                println!("No such luck. The secret was {secret}\n");
                if !play_again() {
                    break;
                }
            }
            GameState::GameWon(turns_taken) => {
                println!(
                    "\nYou won in {}!\n",
                    pluralize("guess", turns_taken.into(), true)
                );
                if !play_again() {
                    break;
                }
            }
            GameState::ExitGame => {
                println!("\nThanks for playing! The secret was {secret}\n");
                break;
            }
        }
    }
}

fn play_game(limit: u8, secret: String) -> GameState {
    for turns_left in (0..limit).rev() {
        println!(
            "\nYou have {: >2} {} left. (type 'quit' to exit)",
            (1 + turns_left),
            pluralize("turn", (1 + turns_left).into(), false)
        );

        let input = read_line("Guess: ").unwrap_or_default();
        let guess = input.clone();

        if input == "quit" || input == "exit" {
            return GameState::ExitGame;
        }

        if guess == secret {
            return GameState::GameWon(limit - turns_left);
        } else if turns_left == 0 {
            break;
        } else {
            println!("Hint: {}", show(feedback(guess, secret.clone())));
        }
    }
    GameState::GameOver
}

fn play_again() -> bool {
    loop {
        let question = "Do you want to play again? (y/n): ";
        let answer = read_line(question).unwrap_or_default();
        match answer.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}
