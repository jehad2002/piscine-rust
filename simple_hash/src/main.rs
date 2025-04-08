use simple_hash::*;

// Test string
const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

fn main() {
    // Split the sentence into words and collect them into a vector
    let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
    
    // Pass the owned vector, not a reference
    let frequency_count = word_frequency_counter(words);  // No `&` here

    // Print the frequency count and number of distinct words
    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}
