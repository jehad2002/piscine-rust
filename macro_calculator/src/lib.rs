use json::JsonValue; // استيراد مكتبة json لإنشاء كائن JSON

// تعريف الهيكل المطلوب لكل طعام
pub struct Food {
    pub name: String,                              // اسم الطعام
    pub calories: (String, String),                // السعرات (kJ و kcal كـ نصوص)
    pub fats: f64,                                 // كمية الدهون
    pub carbs: f64,                                // كمية الكربوهيدرات
    pub proteins: f64,                             // كمية البروتين
    pub nbr_of_portions: f64,                      // عدد الحصص
}

// دالة لحساب المجموع الكامل للقيم الغذائية
pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;      // مجموع السعرات
    let mut total_carbs = 0.0;     // مجموع الكربوهيدرات
    let mut total_proteins = 0.0;  // مجموع البروتين
    let mut total_fats = 0.0;      // مجموع الدهون

    for food in foods {
        // نحصل على السعرات بالكالوري من النص "510kcal" => نحولها إلى f64
        if let Some(kcal_str) = food.calories.1.strip_suffix("kcal") {
            if let Ok(kcal) = kcal_str.parse::<f64>() {
                total_cals += kcal * food.nbr_of_portions; // نضربها في عدد الحصص
            }
        }

        // نجمع القيم الغذائية الأخرى بعد ضربها في عدد الحصص
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // تنسيق القيم: تقريبهما إلى منزلتين عشريتين أو منزلة واحدة إن كانت تنتهي بـ 0
    fn format_num(n: f64) -> f64 {
        let rounded = (n * 100.0).round() / 100.0; // تقريب لمنزلتين
        if (rounded * 10.0) % 10.0 == 0.0 {
            (rounded * 10.0).round() / 10.0 // إن كانت تنتهي بـ 0 نحولها لمنزلة واحدة
        } else {
            rounded
        }
    }

    json::object! {
        "cals" => format_num(total_cals),
        "carbs" => format_num(total_carbs),
        "proteins" => format_num(total_proteins),
        "fats" => format_num(total_fats),
    }
}