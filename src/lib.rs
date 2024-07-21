pub mod code {
    pub fn from_string(guess: String) -> Option<String> {
        if guess.len() == 4 && guess.chars().all(is_valid_digit) {
            Some(guess)
        } else {
            None
        }
    }
    fn is_valid_digit(c: char) -> bool {
        "123456".find(c).is_some()
    }
}

pub mod user_input {
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
