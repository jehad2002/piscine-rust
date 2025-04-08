use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut result: HashMap<&str, usize> = HashMap::new();

    for word in words {
        *result.entry(word).or_insert(0) += 1;
    }

    result
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count: usize = 0;
    for &val in frequency_count.values() {
        if val == 1 {
            count += 1
        }
    }
    count
}
