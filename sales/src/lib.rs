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
    // expected public fields
    pub items: Vec<(String, f32)>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart { items: vec![] }
        
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        // Check if the item exists in the store
        if let Some(product) = s.products.iter().find(|(name, _)| name == &ele) {
            // Add the item to the cart
            self.items.push(product.clone());
        } else {
            println!("Item not found in store: {}", ele);
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // Calculate the total price of items in the cart
        let mut total_price = 0.0;
        for item in &self.items {
            total_price += item.1;
        }
        // Return the total price as a vector
        vec![total_price]
    }
}