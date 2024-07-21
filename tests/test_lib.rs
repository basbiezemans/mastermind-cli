use mastermind_cli::code;

#[test]
fn code_from_correct_string() {
    assert!(code::from_string("1234".to_string()).is_some());
    assert!(code::from_string("6543".to_string()).is_some());
}

#[test]
fn code_from_incorrect_string() {
    assert!(code::from_string("123".to_string()).is_none());
    assert!(code::from_string("12345".to_string()).is_none());
    assert!(code::from_string("1237".to_string()).is_none());
    assert!(code::from_string("123x".to_string()).is_none());
    assert!(code::from_string("0123".to_string()).is_none());
}
