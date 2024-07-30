#![allow(dead_code)]

/// Prompt the user and read from standard input.
pub mod user_input {
    use std::io::{stdin, stdout, Write};
    /// Read bytes from the underlying stream until the newline delimiter
    /// is found. Once found, all bytes, including the delimiter, will be
    /// appended to a buffer, trimmed, and returned as a string.
    pub fn read_line(prompt: &str) -> Result<String, std::io::Error> {
        print!("{}", prompt);
        stdout().flush().unwrap(); // write prompt to standard output
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(input.trim_end().to_string())
    }
}

/// Take a secret code and a guess, and return a hint with the number
/// of correctly guessed digits and the number of digits that are not
/// correct but do occur in the secret.
/// This function is symmetric, i.e. f(a, b) = f(b, a).
pub fn feedback(s1: String, s2: String) -> Option<(usize, usize)> {
    let code1 = code::parse(&s1)?;
    let code2 = code::parse(&s2)?;
    let pairs = zip(code1, code2);
    Some((num_correct(pairs.clone()), num_present(unequal(pairs))))
}

/// Format a hint as a sequence of solid ● and empty ○ dots. A ● for each
/// correct digit, and a ○ for each digit that occurs in the secret code
/// but in the wrong position.
pub fn show(hint: Option<(usize, usize)>) -> String {
    match hint {
        Some((correct, present)) => {
            "●".repeat(correct) + "○".repeat(present).as_str()
        }
        None => String::from(
            "please enter 4 digits, \
                where each digit is between 1 and 6, e.g. 1234",
        ),
    }
}

type Digit = char;
type Code = Vec<Digit>;
type Pair = (Digit, Digit);

/// Parse or generate a 4-digit number.
pub mod code {
    use super::Code;
    use rand::distributions::{Distribution, Uniform};

    const DIGITS: &str = "123456";
    const LENGTH: usize = 4;

    pub fn parse(string: &str) -> Option<Code> {
        if !is_valid(string) {
            return None;
        }
        Some(string.chars().collect())
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

    pub fn is_valid(string: &str) -> bool {
        let is_valid_digit = |c| DIGITS.find(c).is_some();
        string.len() == LENGTH && string.chars().all(is_valid_digit)
    }
}

fn tally(acc: (usize, Code)) -> usize {
    acc.0
}

fn zip(a: Code, b: Code) -> Vec<Pair> {
    a.into_iter().zip(b).collect()
}

fn unzip(pairs: Vec<Pair>) -> (Code, Code) {
    pairs.into_iter().unzip()
}

fn unequal(pairs: Vec<Pair>) -> Vec<Pair> {
    pairs.into_iter().filter(|p| p.0 != p.1).collect()
}

fn num_correct(pairs: Vec<Pair>) -> usize {
    pairs.len() - unequal(pairs).len()
}

fn num_present(pairs: Vec<Pair>) -> usize {
    let (guess, secret) = unzip(pairs);
    tally(guess.into_iter().fold((0, secret), count))
}

// Take an accumulator, containing a tally and secret code, and a
// digit. Increment the tally if the digit occurs in the code.
// Return a new accumulator, if applicable with updated tally and
// code.
fn count(acc: (usize, Code), digit: Digit) -> (usize, Code) {
    let (tally, secret) = acc;
    if secret.contains(&digit) {
        (tally + 1, delete(digit, secret))
    } else {
        (tally, secret)
    }
}

// Remove the first occurrence of a given digit from a code
fn delete(x: Digit, xs: Code) -> Code {
    xs.splitn(2, |y| *y == x).collect::<Vec<_>>().concat()
}

/**** TESTS *****************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_code_from_string() {
        assert!(code::parse("1234").is_some());
        assert!(code::parse("02e7").is_none());
        let expect = char_vec("1234");
        assert_eq!(Some(expect), code::parse("1234"));
    }

    #[test]
    fn zip_two_char_vectors() {
        let a = char_vec("123");
        let b = char_vec("134");
        let expect = vec![('1', '1'), ('2', '3'), ('3', '4')];
        assert_eq!(expect, zip(a, b));
    }

    #[test]
    fn unzip_two_char_vectors() {
        let pairs = vec![('1', '1'), ('2', '3'), ('3', '4')];
        let expect = (char_vec("123"), char_vec("134"));
        assert_eq!(expect, unzip(pairs));
    }

    #[test]
    fn filter_unequal_pairs() {
        let pairs = vec![('1', '1'), ('2', '3'), ('3', '4')];
        let expect = vec![('2', '3'), ('3', '4')];
        assert_eq!(expect, unequal(pairs));
    }

    #[test]
    fn number_of_correct_digits() {
        let pairs = vec![('1', '1'), ('2', '3'), ('3', '4')];
        assert_eq!(1, num_correct(pairs));
    }

    #[test]
    fn number_of_present_digits() {
        let pairs = vec![('2', '3'), ('3', '4')];
        assert_eq!(1, num_present(pairs));
    }

    #[test]
    fn delete_char_from_vector() {
        let v1 = char_vec("1234");
        let v2 = char_vec("1334");
        assert_eq!(char_vec("124"), delete('3', v1));
        assert_eq!(char_vec("134"), delete('3', v2));
    }

    #[test]
    fn does_char_occur_in_vector() {
        let t1 = (0, char_vec("34"));
        let t2 = (0, char_vec("324"));
        assert_eq!(0, tally(count(t1, '5')));
        assert_eq!(1, tally(count(t2, '2')));
    }

    #[test]
    fn show_user_hint() {
        assert_eq!("●○", show(Some((1, 1))));
        assert_eq!("please", &show(None)[..6]);
    }

    #[test]
    fn validate_user_guess() {
        assert!(code::is_valid(&code::random()));
        assert!(code::is_valid("0123") == false);
    }

    type TestCase<'a> = (&'a str, &'a str, (usize, usize));

    #[test]
    fn verify_user_feedback() {
        let test_cases: Vec<TestCase> = vec![
            ("1234", "1234", (4, 0)),
            ("6243", "6225", (2, 0)),
            ("5256", "2244", (1, 0)),
            ("1111", "2222", (0, 0)),
            ("6423", "2252", (0, 1)),
            ("6443", "4124", (0, 2)),
            ("6163", "1136", (1, 2)),
            ("1234", "2134", (2, 2)),
        ];
        for test in test_cases {
            let (a, b, hint) = test;
            assert_eq!(Some(hint), feedback(a.to_string(), b.to_string()));
            assert_eq!(Some(hint), feedback(b.to_string(), a.to_string()));
        }
        assert!(feedback("".to_string(), "1234".to_string()).is_none());
        assert!(feedback("1234".to_string(), "qwerty".to_string()).is_none());
    }

    // Helper function for testing
    fn char_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }
}
