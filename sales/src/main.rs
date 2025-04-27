use sales::*;

fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12)
    ]);

    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    // تأكد من تمرير store كوسيط
    println!("{:?}", cart.generate_receipt(&store));  // تمرير store هنا

    println!("{:?}", cart);
}
