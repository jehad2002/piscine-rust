pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    input[0..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize = true;
            result.push(c);
        } else if capitalize {
            result.extend(c.to_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}


pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            }
        })
        .collect()
}
