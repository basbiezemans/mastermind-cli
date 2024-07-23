#![allow(dead_code)]

use rand::distributions::{Distribution, Uniform};

pub mod user_input {
    use std::io::stdin;
    use std::io::Write;
    pub fn read_line(prompt: &str) -> Result<String, std::io::Error> {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Err(e) => Err(e),
            Ok(_) => Ok(input.trim_end().to_string()),
        }
    }
}

pub fn make_secret() -> String {
    const DIGITS: &[u8] = b"123456";
    const LENGTH: usize = 4;
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0..6);
    let rand = |_| {
        let idx = dist.sample(&mut rng);
        DIGITS[idx] as char
    };
    (0..LENGTH).map(rand).collect()
}

pub fn is_valid(guess: String) -> bool {
    let is_valid_digit = |c| {
        "123456".find(c).is_some()
    };
    guess.len() == 4 && guess.chars().all(is_valid_digit)
}

pub fn feedback(guess: String, secret: String) -> (usize, usize) {
    let guess_code = make_code(guess.as_str());
    let secret_code = make_code(secret.as_str());
    let pairs = zip(guess_code, secret_code);
    (num_correct(pairs.clone()), num_present(unequal(pairs)))
}

pub fn show(hint: (usize, usize)) -> String {
    let (correct, present) = hint;
    "●".repeat(correct) + "○".repeat(present).as_str()
}

type Digit = char;
type Code = Vec<Digit>;
type Pair = (Digit, Digit);
type Pairs = Vec<Pair>;

fn make_code(s: &str) -> Code {
    s.chars().collect()
}

fn first(acc: (usize, Code)) -> usize {
    acc.0
}

fn zip(a: Code, b: Code) -> Pairs {
    a.into_iter().zip(b).collect()
}

fn unzip(pairs: Pairs) -> (Code, Code) {
    pairs.into_iter().unzip()
}

fn unequal(pairs: Pairs) -> Pairs {
    pairs.into_iter().filter(|p| p.0 != p.1).collect()
}

fn num_correct(pairs: Pairs) -> usize {
    pairs.len() - unequal(pairs).len()
}

fn num_present(pairs: Pairs) -> usize {
    let (guess, secret) = unzip(pairs);
    let tuple = guess.into_iter().fold((0, secret), count);
    first(tuple)
}

// Count how many times a digit occurs in a code
fn count(acc: (usize, Code), digit: Digit) -> (usize, Code) {
    let (tally, secret) = acc;
    if secret.contains(&digit) {
        (tally + 1, delete(digit, secret))
    } else {
        (tally, secret)
    }
}

// Remove the first occurrence of agiven digit from a code
fn delete(x: Digit, xs: Code) -> Code {
    xs.splitn(2, |y| *y == x).collect::<Vec<_>>().concat()
}

/**** TESTS *****************************************************/

#[test]
fn zip_two_codes() {
    let a: Code = make_code("123");
    let b: Code = make_code("134");
    let expect: Pairs = vec![('1','1'),('2','3'),('3','4')];
    assert_eq!(expect, zip(a, b));
}

#[test]
fn unzip_two_codes() {
    let pairs: Pairs = vec![('1','1'),('2','3'),('3','4')];
    let expect: (Code, Code) = (
        make_code("123"), make_code("134")
    );
    assert_eq!(expect, unzip(pairs));
}

#[test]
fn filter_unequal_pairs() {
    let pairs: Pairs = vec![('1','1'),('2','3'),('3','4')];
    let expect: Pairs = vec![('2','3'),('3','4')];
    assert_eq!(expect, unequal(pairs));
}

#[test]
fn number_of_correct_digits() {
    let pairs: Pairs = vec![('1','1'),('2','3'),('3','4')];
    assert_eq!(1, num_correct(pairs));
}

#[test]
fn number_of_present_digits() {
    let pairs: Pairs = vec![('2','3'),('3','4')];
    assert_eq!(1, num_present(pairs));
}

#[test]
fn delete_digit_from_code() {
    let c1: Code = make_code("1234");
    let c2: Code = make_code("1334");
    assert_eq!(make_code("124"), delete('3', c1));
    assert_eq!(make_code("134"), delete('3', c2));
}

#[test]
fn count_how_many_times_digit_occurs_in_code() {
    let c1: (usize, Code) = (0, make_code("34"));
    let c2: (usize, Code) = (0, make_code("324"));
    assert_eq!(0, first(count(c1.clone(), '5')));
    assert_eq!(1, first(count(c2.clone(), '2')));
}

#[test]
fn show_user_hint() {
    assert_eq!("●○", show((1,1)));
}

#[test]
fn check_user_guess() {
    assert!(is_valid(make_secret()));
    assert!(is_valid("0123".to_string()) == false);
}

type TestCase<'a> = (&'a str, &'a str, (usize, usize));

#[test]
fn check_user_feedback() {
    let test_cases: Vec<TestCase> = vec![
        ("1234", "1234", (4,0)),
        ("6243", "6225", (2,0)),
        ("5256", "2244", (1,0)),
        ("1111", "2222", (0,0)),
        ("6423", "2252", (0,1)),
        ("6443", "4124", (0,2)),
        ("6163", "1136", (1,2)),
        ("1234", "2134", (2,2)),
    ];
    for test in test_cases {
        let (guess, secret, hint) = test;
        assert_eq!(hint, feedback(guess.to_string(), secret.to_string()));
    }
}
