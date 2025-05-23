use rand::distributions::{Distribution, Uniform};
use std::fmt;

const DIGITS: &str = "123456";
const LENGTH: usize = 4;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    StringTooShort,
    StringTooLong,
    InvalidDigits,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing code error: invalid code")
    }
}

/// Parse a string to a code. The string has to be a 4 digit number,
/// and each digit an element of [1-6].
pub fn parse(string: &str) -> Result<Vec<char>, ParseError> {
    if string.len() < LENGTH {
        return Err(ParseError::StringTooShort);
    }
    if string.len() > LENGTH {
        return Err(ParseError::StringTooLong);
    }
    if !string.chars().all(|c| DIGITS.find(c).is_some()) {
        return Err(ParseError::InvalidDigits);
    }
    Ok(string.chars().collect())
}

/// Generate a random code from a uniform distribution. The generated
/// code is a 4 digit number, and each digit is an element of [1-6].
pub fn random() -> String {
    let bdigits = DIGITS.as_bytes();
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..DIGITS.len());
    let rand = |_| {
        let idx = dist.sample(&mut rng);
        bdigits[idx] as char
    };
    (0..LENGTH).map(rand).collect()
}

/**** TESTS *******************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_code_from_string() {
        assert!(parse(&random()).is_ok());
        assert!(parse("02e7").is_err());
        assert!(parse("123").is_err());
        assert!(parse("12345").is_err());
        let expect = char_vec("1234");
        assert_eq!(Ok(expect), parse("1234"));
    }

    // Helper function for testing
    fn char_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }
}
