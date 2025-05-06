use std::collections::HashMap;

/// تحسب عدد مرات تكرار كل كلمة داخل القائمة
pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();

    for &word in words.iter() {
        *map.entry(word).or_insert(0) += 1;
    }

    map
}

/// تحسب عدد الكلمات المختلفة داخل HashMap
pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}