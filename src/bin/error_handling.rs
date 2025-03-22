fn main() {
    match multiply(5, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// result enums
fn multiply(a: u32, b: u32) -> Result<u32, String> {
    if a == 0 || b == 0 {
        Err(String::from("Multiplication by zero might be unintended"))
    } else {
        Ok(a * b)
    }
}
