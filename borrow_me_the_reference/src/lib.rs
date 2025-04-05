// pub fn delete_and_backspace(s: &mut String) {
//     let mut result = String::new();
//     let chars: Vec<char> = s.chars().collect();
//     let mut i = 0;

//     while i < chars.len() {
//         match chars[i] {
//             '-' => {
//                 result.pop(); // backspace
//                 i += 1;
//             }
//             '+' => {
//                 i += 2; // skip + and char after
//             }
//             c => {
//                 result.push(c);
//                 i += 1;
//             }
//         }
//     }

//     *s = result;
// }

// pub fn do_operations(v: &mut [String]) {
//     for expr in v {
//         let op = if expr.contains('+') {
//             '+'
//         } else {
//             '-'
//         };

//         let parts: Vec<&str> = expr.split(op).collect();
//         if parts.len() != 2 {
//             continue; // skip invalid expressions
//         }

//         let left: i32 = parts[0].parse().unwrap_or(0);
//         let right: i32 = parts[1].parse().unwrap_or(0);

//         let result = match op {
//             '+' => left + right,
//             '-' => left - right,
//             _ => 0,
//         };

//         *expr = result.to_string();
//     }
// }

//=========================================================

pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                result.pop(); // backspace
            }
            '+' => {
                chars.next(); // skip next character (delete)
            }
            _ => result.push(c),
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v {
        let op = if expr.contains('+') { '+' } else { '-' };
        let parts: Vec<&str> = expr.split(op).collect();
        let a = parts[0].parse::<i32>().unwrap_or(0);
        let b = parts[1].parse::<i32>().unwrap_or(0);
        let result = if op == '+' { a + b } else { a - b };
        *expr = result.to_string();
    }
}
