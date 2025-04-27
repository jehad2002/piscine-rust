// sales/src/lib.rs

// تعريف هيكل Store الذي يحتوي على قائمة من المنتجات مع أسعارها.
#[derive(Debug)]
pub struct Store {
    products: Vec<(String, f32)>,
}

impl Store {
    // دالة جديدة لإنشاء Store مع قائمة من المنتجات.
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }

    // دالة لإيجاد سعر المنتج باستخدام اسمه.
    pub fn get_price(&self, product_name: &str) -> Option<f32> {
        self.products.iter().find_map(|(name, price)| {
            if name == product_name {
                Some(*price)
            } else {
                None
            }
        })
    }
}

// تعريف هيكل Cart الذي يحتوي على قائمة المنتجات التي تم إضافتها.
#[derive(Debug)]
pub struct Cart {
    items: Vec<String>,
}

impl Cart {
    // دالة جديدة لإنشاء Cart فارغ.
    pub fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    // دالة لإضافة منتج إلى العربة.
    pub fn insert_item(&mut self, store: &Store, product_name: String) {
        if store.get_price(&product_name).is_some() {
            self.items.push(product_name);
        }
    }

    // دالة لإنشاء الإيصال مع الأسعار.
    pub fn generate_receipt(&self, store: &Store) -> Vec<(String, f32)> {
        self.items.iter().filter_map(|item| {
            store.get_price(item).map(|price| (item.clone(), price))
        }).collect()
    }
}
