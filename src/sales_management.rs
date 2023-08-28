use crate::{SALES_TRANSACTIONS, inventory_management::{read_input_f64, read_input_u32, read_input_usize}, INVENTORY};

pub struct SalesTransaction {
    product_id: u32, // The ID of the product sold
    quantity_sold: u32,
    sale_price: f64,
    _average_sale_price: f64,
}

impl SalesTransaction {
    pub fn new(product_id: u32, quantity_sold: u32, sale_price: f64,_average_sale_price: f64) -> SalesTransaction {
        SalesTransaction {
            product_id,
            quantity_sold,
            sale_price,
            _average_sale_price,
        }
    }

    pub fn calculate_total(&self) -> f64 {
        self.quantity_sold as f64 * self.sale_price
    }
}
pub fn manage_sales() {
    loop {
        println!("Manage Sales:");
        println!("1. Record Sale");
        println!("2. View Sales");
        println!("3. Go Back");

        let choice = read_input_u32();

        match choice {
            1 => record_sale(),
            2 => view_sales(),
            3 => {
                println!("Returning to the main menu.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

pub fn record_sale() {
    println!("Recording a sale...");

    // Display available products for sale
    println!("Available Products:");
    for (index, product) in unsafe { INVENTORY.iter().enumerate() } {
        println!("Product Index: {}", index);
        println!("Product Name: {}", product.name);
        println!("Price: ${:.2}", product.average_cost()); // Use average cost here as the sale price
        println!("Quantity in Stock: {}", product.quantity);
        println!("-----------------------");
    }

    println!("Enter the product index to sell:");
    let product_index = read_input_usize();

    if product_index < unsafe { INVENTORY.len() } {
        let product = unsafe { &mut INVENTORY[product_index] };

        println!("Enter quantity sold:");
        let quantity_sold = read_input_u32();

        if product.quantity >= quantity_sold {
            // Use average cost as the sale price
            let sale_price = read_input_f64();

            // Calculate the total sale price before creating the SalesTransaction
            let total_price = quantity_sold as f64 * sale_price;

            // Calculate the average sale price and store it in the SalesTransaction
            let average_sale_price = product.average_sale_price();
            let sale = SalesTransaction::new(product_index as u32, quantity_sold, sale_price, average_sale_price);

            unsafe {
                SALES_TRANSACTIONS.push(sale);
            }

            // Update the product's quantity in the inventory
            product.quantity -= quantity_sold;
            println!("Sale recorded successfully.");
            println!("Total Sale Price: ${:.2}", total_price); // Display the pre-calculated total price
        } else {
            println!("Insufficient quantity in inventory. Sale not recorded.");
        }
    } else {
        println!("Invalid product index. Product not found in inventory. Sale not recorded.");
    }
}





pub fn view_sales() {
    println!("Sales Transactions:");
    let mut total_sales = 0.0;
    let mut total_profit = 0.0;

    for (index, sale) in unsafe { SALES_TRANSACTIONS.iter().enumerate() } {
        println!("Transaction Index: {}", index);
        println!("Product ID: {}", sale.product_id);
        println!("Quantity Sold: {}", sale.quantity_sold);
        println!("Sale Price: ${:.2}", sale.sale_price);

        let total_sale_price = sale.calculate_total();
        println!("Total Sale Price: ${:.2}", total_sale_price);
        
        // Calculate profit for this sale (assuming cost price is available)
        let product = unsafe { &INVENTORY[sale.product_id as usize] };
        let cost_price = product.price; // Assuming product price is the cost price for simplicity
        let profit = total_sale_price - (sale.quantity_sold as f64 * cost_price);
        println!("Profit: ${:.2}", profit);

        println!("-----------------------");

        // Accumulate total sales and profit
        total_sales += total_sale_price;
        total_profit += profit;
    }

    println!("Total Sales: ${:.2}", total_sales);
    println!("Total Profit: ${:.2}", total_profit);
}
