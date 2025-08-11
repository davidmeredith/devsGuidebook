// Rust Parametric Polymorphism, used in Dave's Dev Guidebook.
use std::fmt;

// Printable trait with default and required methods
trait Printable {
    // Default method implementation
    fn pretty_print(&self) -> String {
        format!("[Default pretty_print: {:?}]", self.format())
    }
    // Required method to implement
    fn format(&self) -> String;
}

// Serializable trait with default methods
trait Serializable {
    fn serialize(&self) -> String;

    // Default validation method
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

// Custom error type for validation
#[derive(Debug)]
struct ValidationError {
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

// Product struct implementing multiple traits
#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: i32,
}

impl Printable for Product {
    // Implementing required format method
    fn format(&self) -> String {
        format!(
            "{} (Price: ${:.2}, Quantity: {})",
            self.name, self.price, self.quantity
        )
    }

    // Optional override of pretty_print
    fn pretty_print(&self) -> String {
        format!("[Product: {}]", self.format())
    }
}

impl Serializable for Product {
    fn serialize(&self) -> String {
        format!(
            "Product{{name={},price={:.2},quantity={}}}",
            self.name, self.price, self.quantity
        )
    }

    // Custom validation implementation
    fn validate(&self) -> Result<(), ValidationError> {
        if self.price < 0.0 {
            return Err(ValidationError {
                message: "Price cannot be negative".to_string(),
            });
        }
        if self.quantity < 0 {
            return Err(ValidationError {
                message: "Quantity cannot be negative".to_string(),
            });
        }
        Ok(())
    }
}

// Display trait for pretty printing
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

// Inventory management trait
trait InventoryManager {
    fn is_low_stock(&self, threshold: i32) -> bool;
    fn restock(&mut self, amount: i32);
}

impl InventoryManager for Product {
    fn is_low_stock(&self, threshold: i32) -> bool {
        self.quantity < threshold
    }

    fn restock(&mut self, amount: i32) {
        self.quantity += amount;
        println!("Restocked {} by {} units", self.name, amount);
    }
}

// Extension trait for string manipulation
trait StringExt {
    fn truncate(&self, max_length: usize) -> String;
    fn word_count(&self) -> usize;
}

impl StringExt for str {
    fn truncate(&self, max_length: usize) -> String {
        if self.len() <= max_length {
            self.to_string()
        } else {
            format!("{}...", &self[..max_length])
        }
    }

    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }
}

// Generic filter function similar to Go's FilterItems
fn filter_items<T, F>(items: &[T], predicate: F) -> Vec<T>
where
    // Trait bounds used to constrain the generics F and T
    F: Fn(&T) -> bool,
    T: Clone,
{
    items
        .iter()
        .filter(|&item| predicate(item))
        .cloned()
        .collect()
}

fn main() {
    // Create products
    let laptop = Product {
        name: "MacBook Pro".to_string(),
        price: 1999.99,
        quantity: 5,
    };

    let keyboard = Product {
        name: "Mechanical Keyboard".to_string(),
        price: 129.99,
        quantity: 2,
    };

    // Demonstrate trait methods
    println!("Pretty Print: {}", laptop.pretty_print());
    println!("Serialized: {}", laptop.serialize());

    // Validation demonstration
    match laptop.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => println!("Validation Error: {}", e),
    }

    // Demonstrate extension trait
    // Notice that convenient dot-notation on long_string shows the trait's functions to increase findability.
    let long_string = "This is a very long string that needs truncation";
    println!("Truncated: {}", long_string.truncate(10));
    println!("Word count: {}", long_string.word_count());

    // Demonstrate polymorhpic filtering
    let products = vec![laptop.clone(), keyboard.clone()];
    let low_stock_products = filter_items(&products, |p| p.is_low_stock(3));
    println!("Low Stock Products:");
    for mut p in low_stock_products {
        println!("{} - Quantity: {}", p.name, p.quantity);
        p.restock(10);
    }
}
