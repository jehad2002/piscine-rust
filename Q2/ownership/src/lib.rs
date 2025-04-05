pub fn first_subword(mut s: String) -> String {
    // البحث عن أول حرف كبير أو underscore بعد أول حرف في السلسلة

    let index = match s.chars().skip(1).position(|c| c.is_uppercase() || c == '_') {

        Some(idx) => idx+1, // إضافة 1 لتخطي الحرف الأول
        None => return s,  // إذا لم يتم العثور على حرف كبير أو underscore، نعيد السلسلة الأصلية
    };
        // تقصير السلسلة إلى الموضع المحدد

    s.truncate(index);
    s
}