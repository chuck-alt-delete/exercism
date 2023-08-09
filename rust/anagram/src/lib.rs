use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut sorted_word: Vec<_> = word_lower.chars().collect();
    sorted_word.sort_unstable();

    possible_anagrams.iter().filter_map(|&s| {
        let s_lower = s.to_lowercase();
        let mut chars: Vec<_> = s_lower.chars().collect();
        chars.sort_unstable();

        if &chars == &sorted_word && s_lower != word_lower {
            Some(s)
        } else {
            None
        }
    }).collect()
}
