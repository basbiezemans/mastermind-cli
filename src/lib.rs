/// Codemaker secret and feedback.
pub mod codemaker;

/// Parse or generate a 4-digit number.
mod code;

/// Prompt the user and read from standard input.
pub mod user_input {
    use std::io::{self, Write};
    /// Read bytes from the underlying stream until the newline delimiter
    /// is found. Once found, all bytes, including the delimiter, will be
    /// appended to a buffer, trimmed, and returned as a string.
    pub fn read_line(prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?; // write prompt to standard output
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
}
