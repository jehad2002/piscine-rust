// sales/src/lib.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }

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

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<String>,
    pub receipt: Vec<f32>,  // حقل لحفظ الإيصال
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, ele: String) {
        if let Some(_price) = store.get_price(&ele) {
            self.items.push(ele);
        }
    }

    pub fn generate_receipt(&mut self, store: &Store) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter()
            .filter_map(|item| store.get_price(item))
            .collect();
        
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());  // ترتيب الأسعار

        // تطبيق الخصم "اشترِ ثلاثة، واحصل على واحد مجانًا"
        let mut total_discount = 0.0;

        for chunk in prices.chunks_mut(3) {
            if chunk.len() == 3 {
                let mut min_price = chunk[0];
                chunk[0] = 0.0; // جعل الأقل مجانًا
                for &price in chunk.iter() {
                    if price < min_price {
                        min_price = price;
                    }
                }
            }
        }

        // ضبط الأسعار لتكون دقيقة بمقدار عشريين
        for price in &mut prices {
            *price = (*price * 100.0).round() / 100.0;  // تم تعديل هذه السطر هنا
        }

        // حفظ النتيجة في حقل الإيصال
        self.receipt = prices.clone();
        prices
    }
}
