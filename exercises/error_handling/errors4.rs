// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value < 0 {
            // 如果传入负数，返回 CreationError::Negative
            Err(CreationError::Negative)
        } else if value == 0 {
            // 如果传入零，返回 CreationError::Zero
            Err(CreationError::Zero)
        } else {
            // 如果传入正整数，返回 Ok 包装的 PositiveNonzeroInteger
            Ok(PositiveNonzeroInteger(value as u64))
        }
        
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
