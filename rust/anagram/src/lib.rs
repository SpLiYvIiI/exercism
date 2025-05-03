use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let word_lower_case = word.to_lowercase();
    let word_character_count = get_character_count_lower(word_lower_case.as_str());
    for possible_anagram in possible_anagrams {
        let possible_anagram_lower_case = possible_anagram.to_lowercase();
        let possible_anagram_character_count =
            get_character_count_lower(possible_anagram_lower_case.as_str());
        let mut is_anagram = true;
        possible_anagram_character_count.keys().for_each(|c| {
            if word_lower_case == possible_anagram_lower_case
                || !word_character_count.contains_key(c)
                || word_character_count[c] != possible_anagram_character_count[c]
            {
                is_anagram = false;
                return;
            }
        });
        if is_anagram {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}

pub fn get_character_count_lower(word: &str) -> HashMap<char, i32> {
    let mut word_character_count: HashMap<char, i32> = HashMap::new();
    for char in word.chars() {
        if word_character_count.contains_key(&char) {
            *word_character_count.get_mut(&char).unwrap() += 1;
        } else {
            word_character_count.insert(char, 1);
        }
    }
    word_character_count
}
