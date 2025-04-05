pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                result.pop(); // backspace
            }
            '+' => {
                chars.next(); // skip next char
            }
            _ => {
                result.push(c);
            }
        }
    }

    *s = result.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for expr in v {
        let op = if expr.contains('+') {
            '+'
        } else {
            '-'
        };

        let parts: Vec<&str> = expr.split(op).collect();
        if parts.len() != 2 {
            continue;
        }

        let left: i32 = parts[0].parse().unwrap_or(0);
        let right: i32 = parts[1].parse().unwrap_or(0);

        let result = match op {
            '+' => left + right,
            '-' => left - right,
            _ => 0,
        };

        *expr = result.to_string();
    }
}
