use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in words
        .split_whitespace() // تقسيم النص حسب المسافات
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'')) // إزالة علامات الترقيم
        .filter(|w| !w.is_empty()) // تجاهل الكلمات الفارغة
    {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}
