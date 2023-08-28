
use inventory_management::Product;
use purchase_management::PurchaseTransaction;
use sales_management::SalesTransaction;

use crate::{inventory_management::{AccountManager, read_input_u32, manage_products, read_input_string}, sales_management::manage_sales, purchase_management::manage_purchases};

mod inventory_management;
mod sales_management;
mod purchase_management;


static mut INVENTORY: Vec<Product> =vec![];
static mut SALES_TRANSACTIONS: Vec<SalesTransaction> = vec![];
static mut PURCHASE_TRANSACTIONS: Vec<PurchaseTransaction> = vec![];


fn main() {
    const VALID_ACCOUNT_MANAGERS: [AccountManager; 3] = [
        AccountManager::new("user4774", "KnDnzz33"),
        AccountManager::new("user3275", "JKldw66"),
        AccountManager::new("user1191", "MmMMsd21"),
    ];

    println!("Welcome to Star Retail Store!!");
    println!("To use the system we need you credentials.");
    let mut authenticated = false;

    while !authenticated {
        // Prompt the user for their user ID
        println!("Please enter your user ID:");
        let mut input_username = String::new();
        std::io::stdin().read_line(&mut input_username).expect("Failed to read input");
        let input_username = input_username.trim();

        // Prompt the user for their password
        println!("Please enter your password:");
        let mut input_password = String::new();
        std::io::stdin().read_line(&mut input_password).expect("Failed to read input");
        let input_password = input_password.trim();

        // Check if the user's credentials match any valid account manager
        let matching_account_manager = VALID_ACCOUNT_MANAGERS.iter().find(|&account_manager| {
            account_manager.authenticate(input_username, input_password)
        });

        // Check if credentials match and start the program or try again
        match matching_account_manager {
            Some(_) => {
                println!("Authentication successful! Starting the program...");
                // Call your program's start function here.
                start();
                authenticated = true; // Exit the loop
            }
            None => {
                println!("Authentication failed. Please try again.");
    
                // Prompt the user to try again or exit
                println!("Do you want to try again? (yes/no)");
                let retry_input = read_input_string().to_lowercase();
    
                if retry_input != "yes" {
                    println!("Exiting the program.");
                    break; // Exit the loop and program
                }
            }
    
    }
fn start(){
    loop {
        println!("Choose an option:");
        println!("1. Manage Products");
        println!("2. Manage Sales"); 
        println!("3. Manage Purchases"); 
        println!("4. Exit");

        let choice = read_input_u32();

        match choice {
            1 => manage_products(),
            2 => manage_sales(),
            3 => manage_purchases(),
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

}
}