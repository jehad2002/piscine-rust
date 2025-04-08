pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    // example: "hello" => "Hello"
    input[0..1].to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new(); // example: "hello world" => "Hello World"
    let mut cap = true;

    for c in input.chars() {
        if c.is_whitespace() {
            cap = true;
            result.push(c);
        } else if cap {
            result.extend(c.to_uppercase());
            cap = false;
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
