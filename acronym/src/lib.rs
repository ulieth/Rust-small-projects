pub fn abbreviate(phrase: &str) -> String {
    let cleaned = phrase.replace('\'', "");

    // Then split on non-alphabetic characters and collect first letters
    cleaned
        .split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
        .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
        .collect()
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
}
