use std::{fmt::Debug, ops::Mul};

// trait bounding using the multiple params
fn level_1<T>(a: T, b: T)
where
    T: Mul<Output = T> + Debug,
{
    let result = a * b;
    println!(
        "mul of a and b would be {:?} -> (trait bounding using the multiple params)",
        result
    );
}

struct Car {
    brand: String,
    speed: u32,
}

impl Car {
    fn new(brand: &str, speed: u32) -> Car {
        Car {
            brand: String::from(brand),
            speed,
        }
    }

    fn print_data(&self) {
        println!(
            "{} is moving at {} km/h -> (struct implementation)",
            self.brand, self.speed
        );
    }
}

trait Description {
    fn describe(&self) -> String;
}

#[derive(Debug)]
struct Bus<T1, T2> {
    brand: T1,
    seats: T2,
}

impl<T1, T2> Description for Bus<T1, T2>
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    fn describe(&self) -> String {
        format!("{} has {} seats inside it", self.brand, self.seats)
    }
}

fn implementing_traits<T: Description>(elem: T) {
    println!("{}", elem.describe());
}

fn passing_traits() {
    let usage = Bus {
        brand: String::from("Tesla"),
        seats: 89,
    };
    implementing_traits(usage);
}


// enum implementation
enum ContractResponse<T, E> {
    Success(T),
    Error(E),
}
impl<T, E> ContractResponse<T, E> {
    fn is_success(&self) -> bool {
        matches!(self, ContractResponse::Success(_))
    }

    fn is_error(&self) -> bool {
        matches!(self, ContractResponse::Error(_))
    }
}

fn process_transaction(amount: i32) -> ContractResponse<String, String> {
    let response = if amount > 0 {
        println!("enum implementation  ->");
        ContractResponse::Success(format!(
            "Transaction of {} processed successfully!", amount
        ))
    } else {
        ContractResponse::Error("Transaction can't be 0".to_string())
    };

    // ✅ Using is_success() and is_error()
    if response.is_success() {
        println!("✔️ Confirmed: Transaction was successful.");
    } else if response.is_error() {
        println!("⚠️ Warning: Transaction failed.");
    }

    response // Return the response
}

fn main() {
    level_1(5.0, 6.0);

    let my_car = Car::new("Tesla", 120);
    my_car.print_data();

    passing_traits();

    let response = process_transaction(100);

    match response {
        ContractResponse::Success(msg) => println!("{}", msg),
        ContractResponse::Error(err) => println!("{}", err),
    }
}
