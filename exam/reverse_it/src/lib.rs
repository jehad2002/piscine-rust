// pub fn reverse_it(v: i32) -> String {
//     let x = v.abs().to_string(); // نحصل على القيمة المطلقة كـ String
//     let reversed: String = x.chars().rev().collect(); // نعمل لها reverse

//     if v < 0 {
//         format!("-{}{}", reversed, x) // إذا الرقم سلبي، نضيف -
//     } else {
//         format!("{}{}", reversed, x) // إذا الرقم موجب
//     }
// }


pub fn reverse_it(v: i32) -> String {
    let mut x = v;
    let mut y = 0;
    let mut sign = 1;
    if v < 0 {
        sign = -1;
        x *= sign;
    }
    let mut steps = 1;
    while x != 0 {
        y *= 10;
        steps *=10;
        y += x%10;
        x /= 10;
    }

    (y*sign*steps + v).to_string()
}
