// https://www.codewars.com/kata/53369039d7ab3ac506000467

pub fn boolean_to_word(value: bool) -> &'static str {
  match value {
      true => "Yes",
      false => "No"
  }
}

#[cfg(test)]
mod tests {
    use super::boolean_to_word;

    #[test]
    fn example_tests() {
        assert_eq!(boolean_to_word(true), "Yes");
        assert_eq!(boolean_to_word(false), "No");
    }
}
