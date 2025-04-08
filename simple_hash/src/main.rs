use simple_hash::*;

const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

fn main() {
    let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>(); // Collect words into a vector
    let frequency_count = word_frequency_counter(words); // Pass the vector directly (not a reference)

    println!("{:?}", frequency_count);
    println!("{}", nb_distinct_words(&frequency_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency_counter() {
        let words = vec!["this", "is", "a", "test"];
        let frequency_count = word_frequency_counter(words);
        let expected = {
            let mut map = std::collections::HashMap::new();
            map.insert("this", 1);
            map.insert("is", 1);
            map.insert("a", 1);
            map.insert("test", 1);
            map
        };
        assert_eq!(frequency_count, expected);
    }

    #[test]
    fn test_nb_distinct_words() {
        let words = vec!["this", "is", "a", "test", "test"];
        let frequency_count = word_frequency_counter(words);
        assert_eq!(nb_distinct_words(&frequency_count), 3);
    }

    #[test]
    fn test_empty() {
        assert_eq!(nb_distinct_words(&word_frequency_counter((&[]).to_vec())), 0); // Fix: Convert empty slice to Vec
    }
}
