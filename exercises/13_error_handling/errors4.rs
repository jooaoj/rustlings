#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        /* if value > 0 {
            Ok(Self(value as u64))
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        } */
       match value.cmp(&0) {
           std::cmp::Ordering::Greater => Ok(Self(value as u64)),
           std::cmp::Ordering::Less => Err(CreationError::Negative),
           std::cmp::Ordering::Equal => Err(CreationError::Zero),
       }
    }
}

fn main() {
    let _positive = PositiveNonzeroInteger::new(64);
    let _negative = PositiveNonzeroInteger::new(-1);
    let _zero = PositiveNonzeroInteger::new(0);

    println!("{:?}", _positive.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
