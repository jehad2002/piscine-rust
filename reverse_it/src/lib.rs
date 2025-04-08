// pub fn reverse_it(v: i32) -> String {
//     let sign = if v < 0 { "-" } else { "" };
//     let binding = v.to_string();
//     let num = binding.trim_matches(|c| c == '-');
//     let reverse = num.to_string().chars().rev().collect::<String>();
//     format!("{}{}{}", sign, reverse, num)
// }

//=====================================

pub fn reverse_it(v: i32) -> String {
    let mut x = v;
    let mut y :i32 = 0;
    let mut sign = 1;
    if v < 0 {
        sign = -1;
        x *= sign;
    }
    while x != 0 {
        y *= 10;
        y += x % 10;
        x /= 10;
        //println!("y = {y}, x = {x}");
    }
    y *= sign;
    y.to_string()
}
