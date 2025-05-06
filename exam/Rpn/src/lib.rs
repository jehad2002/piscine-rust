// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     rpn(&args[1]);
// }

// pub fn rpn(s: &str) {
//     if s.is_empty() {
//         println!("Error");
//         return;
//     }
//     let mut res = Vec::new();
//     for v in s.split_whitespace() {
//         if let Ok(n) = v.parse::<i64>() {
//             res.push(n);
//         } else {
//             if res.len() >= 2 {
//                 let second = res.pop().unwrap();
//                 let first = res.pop().unwrap();
//                 match v {
//                     "+" => res.push(first + second),
//                     "-" => res.push(first - second),
//                     "*" => res.push(first * second),
//                     "/" => res.push(first / second),
//                     "%" => res.push(first % second),
//                     _  => {println!("Error"); return;}
//                 };
//             } else {
//                 println!("Error"); 
//                 return;
//             }
//         }
//     }
//     if res.len() > 1 {
//         println!("Error"); 
//         return;
//     }
//     println!("{}", res[0]);
// }
//========================================================
pub fn rpn(expr: &str) {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut stack: Vec<i64> = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<i64>() {
            stack.push(num);
        } else {
            if stack.len() < 2 {
                println!("Error");
                return;
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    a / b
                }
                "%" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    a % b
                }
                _ => {
                    println!("Error");
                    return;
                }
            };

            stack.push(result);
        }
    }

    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}

