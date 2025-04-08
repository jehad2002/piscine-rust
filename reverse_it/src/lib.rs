// pub fn reverse_it(v: i32) -> String {
//     let z = if v < 0 { "-" } else { "" };
//     let binding = v.to_string();
//     let num = binding.trim_matches(|c| c == '-');
//     let reverse = num.to_string().chars().rev().collect::<String>();
//     format!("{}{}{}", z, reverse, num)
// }

//=====================================

pub fn reverse_it(v: i32) -> String {
    let mut x = v;
    let mut y :i32 = 0;
    let mut z = 1;
    if v < 0 {
        z = -1;
        x *= z;
    }
    while x != 0 {
        y *= 10;
        y += x % 10;
        x /= 10;
        //println!("y = {y}, x = {x}");
    }
    y *= z;
    y.to_string()
}
