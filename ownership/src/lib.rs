pub fn first_subword(mut s: String) -> String {

    let index = match s.chars().skip(1).position(|c| c.is_uppercase() || c == '_') {
        Some(idx) => idx+1,
        None => return s, 
    };
    s.truncate(index);
    s
}