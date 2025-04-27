// main.rs

use sales::*;  // استيراد مكتبة sales

fn main() {
    // إنشاء متجر يحتوي على قائمة من المنتجات وأسعارها.
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12),
    ]);

    // طباعة المعلومات الخاصة بالمتجر.
    println!("{:?}", store);

    // إنشاء عربة تسوق فارغة.
    let mut cart = Cart::new();
    
    // إضافة منتجات إلى العربة.
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    // طباعة الإيصال مع تمرير المتجر كوسيلة
    println!("{:?}", cart.generate_receipt(&store));

    // طباعة العربة نفسها.
    println!("{:?}", cart);
}
