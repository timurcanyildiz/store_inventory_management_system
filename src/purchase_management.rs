use crate::{PURCHASE_TRANSACTIONS, inventory_management::{read_input_u32, read_input_usize, read_input_f64}, INVENTORY};

pub struct PurchaseTransaction {
    product_id: u32, // The ID of the product purchased
    quantity_purchased: u32,
    purchase_price: f64,
}
impl PurchaseTransaction {
    fn new(product_id: u32, quantity_purchased: u32, purchase_price: f64) -> PurchaseTransaction {
        PurchaseTransaction {
            product_id,
            quantity_purchased,
            purchase_price,
        }
    }

    fn calculate_total_cost(&self) -> f64 {
        self.quantity_purchased as f64 * self.purchase_price
    }
}
pub fn manage_purchases() {
    loop {
        println!("Manage Purchases:");
        println!("1. Record Purchase");
        println!("2. View Purchases");
        println!("3. Go Back");

        let choice = read_input_u32();

        match choice {
            1 => record_purchase(),
            2 => view_purchases(),
            3 => {
                println!("Returning to the main menu.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

pub fn record_purchase() {
    println!("Recording a purchase...");

    // Display available products for purchase
    println!("Available Products:");
    for (index, product) in unsafe { INVENTORY.iter().enumerate() } {
        println!("Product Index: {}", index);
        println!("Product Name: {}", product.name);
        println!("Price: ${:.2}", product.average_cost()); // Use average cost here
        println!("Quantity in Stock: {}", product.quantity);
        println!("-----------------------");
    }

    println!("Enter the product index to purchase:");
    let product_index = read_input_usize();

    if product_index < unsafe { INVENTORY.len() } {
        let product = unsafe { &mut INVENTORY[product_index] };

        println!("Enter quantity purchased:");
        let quantity_purchased = read_input_u32();

        println!("Enter purchase price per unit:");
        let purchase_price = read_input_f64();

        // Update the product's purchase price and quantity
        product.total_purchase_cost += purchase_price * quantity_purchased as f64;
        product.total_quantity_purchased += quantity_purchased;
        product.quantity += quantity_purchased;

        let total_cost = quantity_purchased as f64 * purchase_price;

        let purchase = PurchaseTransaction::new(product_index as u32, quantity_purchased, purchase_price);
        unsafe {
            PURCHASE_TRANSACTIONS.push(purchase);
        }

        println!("Purchase recorded successfully.");
        println!("Total Cost: ${:.2}", total_cost);
    } else {
        println!("Invalid product index. Product not found in inventory. Purchase not recorded.");
    }
}



pub fn view_purchases() {
    println!("Purchase Transactions:");
    for (index, purchase) in unsafe { PURCHASE_TRANSACTIONS.iter().enumerate() } {
        println!("Transaction Index: {}", index);
        println!("Product ID: {}", purchase.product_id);
        println!("Quantity Purchased: {}", purchase.quantity_purchased);
        println!("Purchase Price per Unit: ${:.2}", purchase.purchase_price);

        let total_cost = purchase.calculate_total_cost();
        println!("Total Cost: ${:.2}", total_cost);

        println!("-----------------------");
    }
}
