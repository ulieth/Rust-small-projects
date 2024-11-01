pub fn abbreviate(phrase: &str) -> String {
  let mut result = String::new();
   let cleaned = phrase.replace('\'', "");
   let chars: Vec<char> = cleaned.chars().collect();

   // Add first character if string is not empty
   if let Some(&first) = chars.first() {
       result.push(first.to_ascii_uppercase());
   }

   // Process rest of the characters
   for i in 1..chars.len() {
       let current = chars[i];
       let prev = chars[i - 1];

       // Add character if:
       // 1. Previous char is space/non-alphabetic and current is alphabetic
       // 2. Current is uppercase and previous is lowercase (camelCase)
       if (!prev.is_alphabetic() && current.is_alphabetic()) ||
          (current.is_uppercase() && prev.is_lowercase()) {
           result.push(current.to_ascii_uppercase());
       }
   }

   result
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

    #[test]
    fn apostrophes() {
        let input = "Halley's Comet";
        let output = abbreviate(input);
        let expected = "HC";
        assert_eq!(output, expected);
    }

    #[test]
    fn camelcase() {
        let input = "HyperText Markup Language";
        let output = abbreviate(input);
        let expected = "HTML";
        assert_eq!(output, expected);
    }
}
