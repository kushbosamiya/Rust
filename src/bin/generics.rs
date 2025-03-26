use std::{fmt::Debug, ops::Mul};

fn main() {
    level_1(5, 6);
    //
    let my_car = Car::new("Tesla", 120);
    my_car.print_data(); // Output: Tesla is moving at 120 km/h
    
    // implementing the traits 
    passing_traits();

}

fn level_1<T>(a: T, b: T) // Generic function accepting two params of type T, utilizing parametric polymorphism
where
    T: Mul<Output = T> + Debug,
    // Step 2: Type Constraints with Where Clause
    // This restricts what types can be used with this function:
    // multiplication with result of same type
    // - Debug: The type must be printable with the debug formatter
{
    // Step 3: Function Body - Multiplication
    // This multiplies the two parameters using the * operator
    // This works because we required T to implement the Mul trait
    let result = a * b;

    println!("mul of a and b would be {:?}", result);
}

// basic struct implementation

// Step 1: Define the struct with its fields and types
struct Car {
    brand: String,
    speed: u32,
}

// Step 2: Create an implementation block using impl
impl Car {
    // Step 3: Add constructors like new() to create instances
    fn new(brand: &str, speed: u32) -> Car {
        Car {
            brand: String::from(brand),
            speed,
        }
    }

    // Step 4: Add methods that operate on the struct
    fn print_data(&self) {
        println!("{} is moving at {} km/h", self.brand, self.speed);
    }
}

// Step 5: Use the struct by creating instances
// let my_car = Car::new("Tesla", 120);

// Step 6: Call methods on the instances
// my_car.print_data();



// implementing traits 



// Define the trait first
trait Description {
    fn describe(&self) -> String;
}

// Step 2: Define a struct
struct bus
{
  brand:String,
  seats:u32
}

// Step 3: Implement the trait for the struct
impl Description for bus {
    fn describe(&self) -> String  // Error 2: The function should return a String
    {
      format!("{} has {} seats inside it", self.brand, self.seats)
    }
}
// Step 4: Use the trait in a function
fn implementing_traits<T:Description>(elem:T)
{
println!("{}",elem.describe());
}

// passing the traits to thr main function
fn passing_traits()
{
  let usage = bus
{
  brand:String::from("Volvo"),
  seats:89,
};
implementing_traits(usage);
}