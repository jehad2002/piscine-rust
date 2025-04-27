// lib.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    // Initializes the store with a list of products and their prices
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>, // Items in the cart
    pub receipt: Vec<f32>,         // Receipt after applying discounts
}

impl Cart {
    // Initializes a new, empty cart
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    // Inserts an item into the cart based on the store's products
    pub fn insert_item(&mut self, store: &Store, item_name: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| name == &item_name) {
            self.items.push(product.clone());
        }
    }

    // Generates the receipt with the promotion applied
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // First, sort the items by price in ascending order
        let mut sorted_items: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Apply the promotion: every three items, the cheapest is free
        let mut total_discount = 0.0;
        let mut i = 0;

        // Calculate the total discount for groups of three
        while i + 2 < sorted_items.len() {
            total_discount += sorted_items[i]; // The cheapest item in the group is free
            i += 3; // Move to the next group of three
        }

        // Now we will apply the discount proportionally to all items
        let total_sum: f32 = sorted_items.iter().sum();
        let discount_percentage = 1.0 - (total_discount / total_sum);

        // Adjust all item prices with the discount and apply rounding to two decimals
        let adjusted_items: Vec<f32> = sorted_items
            .into_iter()
            .map(|price| (price * discount_percentage).round() * 100.0 / 100.0) // Round to two decimals
            .collect();

        // Save the receipt
        self.receipt = adjusted_items.clone();
        adjusted_items
    }
}
