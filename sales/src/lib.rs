// lib.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
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
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, item_name: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| name == &item_name) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // First, sort the items by price in ascending order
        let mut sorted_items: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Apply the "Buy 3, Get 1 Free" promotion: The cheapest of each group of three is free
        let mut i = 0;
        while i + 2 < sorted_items.len() {
            sorted_items[i] = 0.0; // Set the cheapest item in each group to zero
            i += 3; // Move to the next group of three
        }

        // Apply rounding to two decimals
        let adjusted_items: Vec<f32> = sorted_items
            .into_iter()
            .map(|price| (price * 100.0).round() / 100.0) // Round to two decimals
            .collect();

        // Save the receipt
        self.receipt = adjusted_items.clone();
        adjusted_items
    }
}
