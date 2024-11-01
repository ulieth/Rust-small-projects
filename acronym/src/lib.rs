pub fn abbreviate(phrase: &str) -> String {
  // Split the phrase into words and collect the first letter of each word
  let acronym: String = phrase
      .split_whitespace() // Split by whitespace
      .filter_map(|word| {
          // Use filter_map to ignore empty words and collect the first letter
          word.chars().next()
      })
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
}
