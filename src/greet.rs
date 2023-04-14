pub fn greet() -> &'static str {
  "hello world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greets_the_world() {
        assert_eq!(greet(), "hello world!", "should return the correct message");
    }
}
