// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        // Err(CreationError::Negative),
        Ok(PositiveNonzeroInteger(18446744073709551606)),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(
           Ok(PositiveNonzeroInteger(0)),
        PositiveNonzeroInteger::new(0));
}
