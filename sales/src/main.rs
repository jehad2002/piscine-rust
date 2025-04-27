// main.rs

use sales::*; // This imports the lib.rs file where the Store and Cart structs are defined.

fn main() {
    // Creating a store with some products
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12),
    ]);

    // Printing the store details
    println!("{:?}", store);

    // Creating a cart and adding items from the store to the cart
    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    // Generating the receipt with the applied promotion
    let receipt = cart.generate_receipt();

    // Printing the receipt
    println!("{:?}", receipt);

    // Printing the cart with items and the receipt
    println!("{:?}", cart);
}
