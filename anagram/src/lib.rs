use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();
    let mut anagrams = HashSet::new();
    for anagram in possible_anagrams {
        let mut sorted_anagram = anagram.to_lowercase().chars().collect::<Vec<char>>();
        sorted_anagram.sort_unstable();
        if sorted_word == sorted_anagram && word.to_lowercase() != anagram.to_lowercase() {
            anagrams.insert(*anagram);
        }
    }
    anagrams
}
