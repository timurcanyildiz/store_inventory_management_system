use crate::INVENTORY;
use rand::Rng;
pub struct AccountManager{
    id: &'static str,
    password: &'static str,
}
impl AccountManager {
    pub const fn new(id: &'static str, password:&'static str) -> AccountManager {
        AccountManager{
            id,
            password
        }
    }
    pub fn authenticate(&self, id: &str, password: &str) -> bool {
        self.id == id && self.password == password
    }
}

pub struct Product{
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
    pub total_purchase_cost: f64, // New field for total purchase cost
    pub total_quantity_purchased: u32, // New field for total quantity purchased
    pub total_quantity_sold: u32, // New field for total quantity sold
    pub total_sale_cost: f64,
}
impl Product {
    fn new(name: &str, description: &str) -> Product {
        let mut rng = rand::thread_rng();
        let id: String = format!("#{:04}", rng.gen_range(0..10000));

        Product {
            id, 
            name: name.to_string(),
            description: description.to_string(),
            price: 0.0,
            quantity: 0,
            total_purchase_cost: 0.0,
            total_quantity_purchased: 0,
            total_quantity_sold: 0,
            total_sale_cost: 0.0
        }
    }
    
    
    // Implement an update method to modify product details
   
    pub fn average_cost(&self) -> f64 {
        if self.total_quantity_purchased > 0 {
            self.total_purchase_cost / self.total_quantity_purchased as f64
        } else {
            0.0
        }
    }
    pub fn average_sale_price(&self) -> f64 {
        if self.total_quantity_sold > 0 {
            self.total_sale_cost / self.total_quantity_sold as f64
        } else {
            0.0
        }
    }

}
pub fn add_product() {
    println!("Adding a new product...");
    println!("Enter product name:");
    let name = read_input_string();

    println!("Enter product description:");
    let description = read_input_string();

    let product = Product::new(&name, &description);
    unsafe {
        INVENTORY.push(product);
    }
    println!("Product added successfully.");
}
pub fn edit_product() {
    println!("Editing a product...");
    println!("Enter the product ID to edit:");
    let product_id = read_input_string();

    let product_index = unsafe { INVENTORY.iter().position(|product| product.id == product_id) };

    match product_index {
        Some(index) => {
            let product = unsafe { &mut INVENTORY[index] };

            println!("Select property to edit:");
            println!("1. Name");
            println!("2. Description");
            println!("3. ID");

            let choice = read_input_u32();

            match choice {
                1 => {
                    println!("Enter new product name:");
                    let new_name = read_input_string();
                    product.name = new_name;
                    println!("Product name updated successfully.");
                }
                2 => {
                    println!("Enter new product description:");
                    let new_description = read_input_string();
                    product.description = new_description;
                    println!("Product description updated successfully.");
                }
                3 => {
                    println!("Enter new product ID:");
                    let new_id = read_input_string();
                    product.id = new_id;
                    println!("Product ID updated successfully.");
                }
                _ => println!("Invalid choice. No property updated."),
            }
        }
        None => println!("Product not found in inventory. Update not performed."),
    }
}

pub fn delete_product() {
    println!("Deleting a product...");
    println!("Enter the product index to delete:");
    let index = read_input_usize();

    if index < unsafe { INVENTORY.len() } {
        unsafe {
            INVENTORY.remove(index);
        }
        println!("Product deleted successfully.");
    } else {
        println!("Invalid index. Product not found.");
    }
}
pub fn read_input_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
pub fn view_inventory() {
    println!("Inventory:");
    for (index, product) in unsafe { INVENTORY.iter().enumerate() } {
        println!("Product Index: {}", index);
        println!("Product ID: {}", product.id); // Display product ID
        println!("Product Name: {}", product.name);
        println!("Product Description: {}", product.description);
        println!("Price: ${:.2}", product.average_cost()); // Display average cost
        println!("Average Selling Price: ${:.2}", product.average_sale_price()); // Display average sale price
        println!("Quantity in Stock: {}", product.quantity);
        println!("-----------------------");
    }
}

pub fn read_input_usize() -> usize {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().parse().expect("Invalid input. Please enter a number.")
}

pub fn read_input_f64() -> f64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input. Please enter a number.")
}

pub fn read_input_u32() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input. Please enter a number.")
}
pub fn manage_products() {
    loop {
        println!("Manage Products:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. View Inventory");
        println!("5. Go Back");

        let choice = read_input_u32();

        match choice {
            1 => add_product(),
            2 => edit_product(),
            3 => delete_product(),
            4 => view_inventory(),
            5 => {
                println!("Returning to the main menu.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

