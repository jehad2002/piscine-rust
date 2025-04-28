pub fn first_fifty_even_square() -> Vec<i32> {
    (2..)                        // بدء التكرار من 2
        .filter(|x| x % 2 == 0)   // فلترة الأعداد الزوجية
        .map(|x| x * x)            // تربيع الأعداد
        .take(50)                  // أخذ أول 50 عددًا
        .collect()                 // جمع الأعداد في Vec
}
