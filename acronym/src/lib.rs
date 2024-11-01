pub fn abbreviate(phrase: &str) -> String {
  // Split the phrase using non-alphanumeric characters as delimiters
  let acronym: String = phrase
  .split(|c: char| !c.is_alphanumeric()) // Split on non-alphanumeric characters
  .filter_map(|word| word.chars().next()) // Collect the first letter of each word
  .map(|c| c.to_ascii_uppercase()) // Convert to uppercase
  .collect(); // Collect into a String

  acronym
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "Portable Network Graphics";
        let output = abbreviate(input);
        let expected = "PNG";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation() {
        let input = "First In, First Out";
        let output = abbreviate(input);
        let expected = "FIFO";
        assert_eq!(output, expected);
    }
}
