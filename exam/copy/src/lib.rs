// pub fn nbr_function(c: i32) -> (i32, f64, f64) {
//     let exp = (c as f64).exp();
//     let ln = if c == 0 {
//         f64::NEG_INFINITY
//     } else {
//         (c as f64).abs().ln()
//     };
//     (c, exp, ln)
// }

// pub fn str_function(a: String) -> (String, String) {
//     let exp_values = a
//         .split_whitespace()
//         .map(|s| s.parse::<f64>().unwrap_or(0.0)) // نحول النص لرقم عشري
//         .map(|x| x.exp()) // نحسب الأس
//         .map(|x| x.to_string()) // نحوله مرة ثانية لنص
//         .collect::<Vec<String>>() // نجمع القيم في vector
//         .join(" "); // نجمعها كسلسلة مفصولة بمسافات
//     (a, exp_values)
// }

// pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
//     let ln_values = b
//         .iter()
//         .map(|&x| (x as f64).abs().ln())
//         .collect::<Vec<f64>>();
//     (b, ln_values)
// }

//==================================================================

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let result = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let result = b.iter().map(|&x| (x.abs() as f64).ln()).collect();
    (b, result)
}
