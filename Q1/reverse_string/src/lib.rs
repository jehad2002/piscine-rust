pub fn rev_str(input: &str) -> String {
    let str_vec : Vec<char> = input.chars().rev().collect();
    return str_vec.into_iter().collect();
}