use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams:&'a [&str]) -> HashSet<&'a str> {
    // Normalize the input word by sorting its characters
    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort();
    let sorted_word_str = sorted_word.iter().collect::<String>();

    // Create a HashSet to store the anagrams
    let mut anagrams = HashSet::new();

    // Check each possible anagram
    for &possible in possible_anagrams {
        // Normalize the possible anagram
        let mut sorted_possible: Vec<char> = possible.to_lowercase().chars().collect();
        sorted_possible.sort();
        let sorted_possible_str = sorted_possible.iter().collect::<String>();

        // If it matches the sorted word, add to the HashSet
        if sorted_word_str == sorted_possible_str {
            anagrams.insert(possible);
        }
    }

    anagrams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn detects_two_anagrams() {
        let word = "solemn";
        let inputs = &["lemons", "cherry", "melons"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter(["lemons", "melons"]);
        assert_eq!(output, expected);
    }
    #[test]
    #[ignore]
    fn words_are_not_anagrams_of_themselves() {
        let word = "BANANA";
        let inputs = &["BANANA"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter([]);
        assert_eq!(output, expected);
    }
}
