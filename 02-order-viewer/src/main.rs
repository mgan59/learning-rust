use std::env;
use std::fs;
use std::process;

use colored::Colorize;
use serde::Deserialize;

// Define a struct that maps to each JSON line in the file
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Order {
    product_name: String,
    quantity: u32,
    date_of_purchase: String,
}

fn main() {
    // Grab the file path from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{} Usage: order_viewer <path-to-file.jsonl>", "Error:".red().bold());
        process::exit(1);
    }

    let file_path = &args[1];

    // Read the entire file into a string
    let contents = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("{} Could not read '{}': {}", "Error:".red().bold(), file_path, err);
            process::exit(1);
        }
    };

    // Parse each line as a JSON object
    let mut orders: Vec<Order> = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<Order>(line) {
            Ok(order) => orders.push(order),
            Err(err) => {
                eprintln!(
                    "{} Failed to parse line {}: {}",
                    "Warning:".yellow().bold(),
                    line_num + 1,
                    err
                );
            }
        }
    }

    if orders.is_empty() {
        println!("{}", "No orders found in file.".yellow());
        return;
    }

    // Print a formatted table header
    println!();
    println!(
        "  {:<30} {:>8} {}",
        "Product".bold().underline(),
        "Qty".bold().underline(),
        "Date".bold().underline()
    );
    println!("  {}", "-".repeat(54).dimmed());

    // Print each order as a formatted row
    for order in &orders {
        let qty_display = if order.quantity > 1 {
            order.quantity.to_string().cyan().bold()
        } else {
            order.quantity.to_string().white()
        };

        println!(
            "  {:<30} {:>8} {}",
            order.product_name.green(),
            qty_display,
            order.date_of_purchase.dimmed()
        );
    }

    // Print a summary
    let total_items: u32 = orders.iter().map(|o| o.quantity).sum();
    println!("  {}", "-".repeat(54).dimmed());
    println!(
        "  {} {} orders, {} total items",
        "Summary:".bold(),
        orders.len().to_string().cyan(),
        total_items.to_string().cyan().bold()
    );
    println!();
}
