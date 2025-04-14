pub fn reverse_it(v: i32) -> String {
    let x = v.abs().to_string(); // نحصل على القيمة المطلقة كـ String
    let reversed: String = x.chars().rev().collect(); // نعمل لها reverse

    if v < 0 {
        format!("-{}{}", reversed, x) // إذا الرقم سلبي، نضيف -
    } else {
        format!("{}{}", reversed, x) // إذا الرقم موجب
    }
}
