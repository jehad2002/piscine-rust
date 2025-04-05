// pub fn initials(names: Vec<&str>) -> Vec<String> {
//     names.iter().map(|name| {
//         name
//             .split_whitespace()
//             .map(|word| format!("{}.", word.chars().next().unwrap()))
//             .collect::<Vec<_>>()
//             .join(" ")
//     }).collect()
// }

//==============================================

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for name in names {
        let mut initials = Vec::new();

        for word in name.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                initials.push(format!("{}.", first_char));
            }
        }

        result.push(initials.join(" "));
    }

    result
}
