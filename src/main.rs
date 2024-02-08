fn main() {
    
}

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32
}


impl Product {
    fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Self {
            name, description, price, quantity
        }
    }
}


struct SalesTransaction {
    product_name: String,
    quantity_sold: u32,
    sale_price = f64,
}


impl SalesTransaction {
    fn new(product_name: String, quantity_sold: u32, sale_price: f64) -> Self {
        Self { 
            product_name, quantity_sold, sale_price 
        }
    }
}


struct PurchaseTransaction {
    product_name: String,
    quantity_purchased: u32,
    purchase_price: f64,
}


impl PurchaseTransaction {
    fn new(product_name, quantity_purchased, purchase_price) -> Self {
        Self {
            product_name, quantity_purchased, purchase_price
        }
    }
}


struct Inventory {
    products: Vec<Product>,
}


impl Inventory {
    fn add_product(&mut self, product: Product){
        self.products.push(product);
    }

    fn edit_product(&mut self, index: usize, new_product: Product) {
        if let Some(product) = self.products.get_mut(index) {
            *product = product_name;
        }
    }

    fn delete_product(&mut self, index: usize) {
        self.products.remove(index);
    }
}