fn main() {
    let mut inventory = Inventory { products: vec![] };

    inventory.add_product(Product::new(
        String::from("Product 1"),
        String::from("Description 1"),
        10.99,
        100,
    ));

    inventory.add_product(Product::new(
        String::from("Product 2"),
        String::from("Description 2"),
        20.49,
        50,
    ));

    let sale_transaction = SalesTransaction::new(String::from("Product 1"), 5, 10.99);
    if let Some(product) = inventory
        .products
        .iter_mut()
        .find(|p| p.name == sale_transaction.product_name)
    {
        if product.quantity >= sale_transaction.quantity_sold {
            product.quantity -= sale_transaction.quantity_sold;
            let total_sale = sale_transaction.quantity_sold as f64 * sale_transaction.sale_price;
            println!("Sale transaction recorded. Total Sale: ${:.2}", total_sale);
        } else {
            println!("Insufficient quantity in stock.");
        }
    } else {
        println!("Product not found");
    }

    let purchase_transaction = PurchaseTransaction::new(String::from("Product 2"), 20, 18.49);
    if let Some(product) = inventory
        .products
        .iter_mut()
        .find(|p| p.name == purchase_transaction.product_name)
    {
        product.quantity += purchase_transaction.quantity_purchased;
        let total_cost =
            purchase_transaction.quantity_purchased as f64 * purchase_transaction.purchase_price;
        println!(
            "Purchase Transaction recorded. Total cost: {:.2}",
            total_cost
        );
    } else {
        println!("Product not found in inventory.")
    }
}

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Self {
            name,
            description,
            price,
            quantity,
        }
    }
}

struct SalesTransaction {
    product_name: String,
    quantity_sold: u32,
    sale_price: f64,
}

impl SalesTransaction {
    fn new(product_name: String, quantity_sold: u32, sale_price: f64) -> Self {
        Self {
            product_name,
            quantity_sold,
            sale_price,
        }
    }
}

struct PurchaseTransaction {
    product_name: String,
    quantity_purchased: u32,
    purchase_price: f64,
}

impl PurchaseTransaction {
    fn new(product_name: String, quantity_purchased: u32, purchase_price: f64) -> Self {
        Self {
            product_name,
            quantity_purchased,
            purchase_price,
        }
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
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
