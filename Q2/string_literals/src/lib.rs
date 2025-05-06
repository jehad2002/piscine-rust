pub fn is_empty(v: &str) -> bool {
    // Check if the string is empty
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    // Check if the string contains only ASCII characters
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    // Check if the string contains the pattern
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    // Split the string at the given index
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    // Find the first occurrence of the character in the string
    v.find(pat).unwrap_or(usize::MAX)
}
