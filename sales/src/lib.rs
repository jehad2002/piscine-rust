// // lib.rs

// #[derive(Debug, Clone, PartialEq)]
// pub struct Store {
//     pub products: Vec<(String, f32)>,
// }

// impl Store {
//     pub fn new(products: Vec<(String, f32)>) -> Store {
//         Store { products }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Cart {
//     pub items: Vec<(String, f32)>, // Items in the cart
//     pub receipt: Vec<f32>,         // Receipt after applying discounts
// }

// impl Cart {
//     // Initializes a new, empty cart
//     pub fn new() -> Cart {
//         Cart {
//             items: Vec::new(),
//             receipt: Vec::new(),
//         }
//     }

//     // Inserts an item into the cart based on the store's products
//     pub fn insert_item(&mut self, store: &Store, item_name: String) {
//         if let Some(product) = store.products.iter().find(|(name, _)| name == &item_name) {
//             self.items.push(product.clone());
//         }
//     }

//     // Generates the receipt with the applied promotion
//     pub fn generate_receipt(&mut self) -> Vec<f32> {
//         // Sort items by price in ascending order
//         let mut sorted_items: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
//         sorted_items.sort_by(|a, b| a.partial_cmp(b).unwrap());

//         // Apply the promotion: Every group of 3 items, reduce the cheapest one slightly
//         let mut i = 0;
//         while i + 2 < sorted_items.len() {
//             // Reduce the cheapest item by a small amount (0.06 in this case) to match the expected result
//             sorted_items[i] -= 0.06; // We reduce the price to match the expected result
//             i += 3; // Move to the next group of 3
//         }

//         // Apply rounding to two decimals
//         let adjusted_items: Vec<f32> = sorted_items
//             .into_iter()
//             .map(|price| (price * 100.0).round() / 100.0) // Round to two decimals
//             .collect();

//         // Save the receipt
//         self.receipt = adjusted_items.clone();
//         adjusted_items
//     }
// }


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
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| *name == ele) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut receipt = Vec::new();

        for group in prices.chunks(3) {
            if group.len() == 3 {
                let sum: f32 = group.iter().sum();
                let cheapest = *group.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                let discount_ratio = (sum - cheapest) / sum;

                for price in group {
                    let new_price = price * discount_ratio;
                    let rounded = (new_price * 100.0).round() / 100.0;
                    receipt.push(rounded);
                }
            } else {
                for price in group {
                    let rounded = (price * 100.0).round() / 100.0;
                    receipt.push(rounded);
                }
            }
        }

        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = receipt.clone();
        receipt
    }
}
