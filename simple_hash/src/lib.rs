use std::collections::HashMap;

// Function to count the frequency of each word in the vector
pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut res = HashMap::new(); // Create a new empty HashMap

    // Iterate over each word in the vector
    for word in words {
        // Insert the word into the HashMap or update the frequency count
        *res.entry(word).or_insert(0) += 1;
    }

    res // Return the frequency map
}

// Function to count the number of distinct words in the frequency map
pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    // Return the count of keys in the HashMap (each key is a distinct word)
    frequency_count.len()
}
