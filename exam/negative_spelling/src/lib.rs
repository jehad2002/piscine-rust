pub fn negative_spell(n: i64) -> String {
    if n >= 0 {
        return "error: positive number".to_string();
    }

    let num = -n; // الحصول على القيمة المطلقة
    let words = number_to_words(num);

    format!("minus {}", words)
}

fn number_to_words(n: i64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=19 => SMALL[n as usize].to_string(),
        20..=99 => {
            let tens = n / 10;
            let remainder = n % 10;
            if remainder == 0 {
                TENS[tens as usize].to_string()
            } else {
                format!("{}-{}", TENS[tens as usize], SMALL[remainder as usize])
            }
        }
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", SMALL[hundreds as usize])
            } else {
                format!("{} hundred {}", SMALL[hundreds as usize], number_to_words(remainder))
            }
        }
        1000..=999_999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} thousand", number_to_words(thousands))
            } else {
                format!("{} thousand {}", number_to_words(thousands), number_to_words(remainder))
            }
        }
        1_000_000 => "one million".to_string(),
        _ => "number too large".to_string(),
    }
}

// ثابتات للأرقام الصغيرة والعشرات
const SMALL: [&str; 20] = [
    "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen",
    "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty",
    "fifty", "sixty", "seventy", "eighty", "ninety"
];
