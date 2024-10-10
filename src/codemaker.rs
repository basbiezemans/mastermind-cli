use crate::code::{self, ParseError};

type Digit = char;
type Code = Vec<Digit>;
type Pair = (Digit, Digit);
type Hint = (usize, usize);

/// Generate a secret code.
pub fn secret() -> String {
    code::random()
}

/// Take a secret code and a guess, and return a hint with the number
/// of correctly guessed digits and the number of digits that are not
/// correct but do occur in the secret.
/// This function is symmetric, i.e. f(a, b) = f(b, a).
pub fn feedback(s1: String, s2: String) -> Result<Hint, ParseError> {
    let code1 = code::parse(&s1)?;
    let code2 = code::parse(&s2)?;
    let pairs = zip(code1, code2);
    Ok((num_correct(pairs.clone()), num_present(unequal(pairs))))
}

/// Format a hint as a sequence of solid ● and empty ○ dots. A ● for each
/// correct digit, and a ○ for each digit that occurs in the secret code
/// but in the wrong position.
pub fn show(hint: Result<Hint, ParseError>) -> String {
    match hint {
        Ok((correct, present)) => {
            "●".repeat(correct) + "○".repeat(present).as_str()
        }
        Err(error) => match error {
            ParseError::StringTooShort | ParseError::StringTooLong => {
                String::from("please enter 4 digits")
            }
            ParseError::InvalidDigits => {
                String::from("each digit should be between 1 and 6, e.g. 1234")
            }
        },
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
fn count((tally, secret): (usize, Code), digit: Digit) -> (usize, Code) {
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

/**** TESTS *******************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("●○", show(Ok((1, 1))));
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
            assert_eq!(Ok(hint), feedback(a.to_string(), b.to_string()));
            assert_eq!(Ok(hint), feedback(b.to_string(), a.to_string()));
        }
        assert!(feedback("".to_string(), "1234".to_string()).is_err());
        assert!(feedback("1234".to_string(), "qwerty".to_string()).is_err());
    }

    // Helper function for testing
    fn char_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }
}
