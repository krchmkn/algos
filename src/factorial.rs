/// Factorial
fn factorial(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }

    x * factorial(x - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_one() {
        let result = factorial(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_five() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}
