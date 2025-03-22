fn ownership()
{
    // Ownership example
	let s1 = String::from("hellow");

	let s2 = &s1;  // Borrowing, ownership transfer nahi
    println!("from s1: {}", s1);  // ✅ Still valid
    println!("from s2: {}", s2);  // ✅ Works

    let s3 = s2;  // Another borrow
    println!("from s3: {}", s3);  // ✅ Works

    // Passing ownership from one function to another
    let s1 = String::from(" s1 Content");
    let s2 = taking_func(s1);  // Ownership moves to function, then comes back
    println!("showcase that it returns the ownership :{}", s2);  // ✅ Works

}

// for returning the ownership - CASE -1
fn taking_func(str: String) -> String {
   return str  // Ownership returned
}

fn main() {
    ownership();
}
