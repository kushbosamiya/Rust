use std::collections::HashMap; 

fn main()
{
level_1();
level_2();
level_3();
}

// level_1 -> basics

fn level_1()
{
    // empty hashmaps
    let mut scores: HashMap<String, i32> = HashMap::new();

    // to insert the values inside the hashmaps

    scores.insert(String::from("Kush"),32);
    scores.insert(String::from("Varun"), 45);
    println!("initial score : -> {:?}",scores);

    // access the scores
    match scores.get("Kush")
    {
        Some(score) => println!("this is my score: {}", score),
        None => println!("this is my default statement"),
    }

    // update the score 
    scores.insert(String::from("Kush"), 89);
    println!("updated score : -> {:?}",scores);

//    remove the score
    scores.remove("Kush");
    println!("after removal of the element : {:?}", scores);

    
    
}

fn level_2()
{
    // capacity 

    let mut fruits = HashMap::with_capacity(3);
    // Add some fruits
    fruits.insert(String::from("Apple"), 5);
    fruits.insert(String::from("Banana"), 3);

    println!("Initial fruits: {:?}", fruits);
    println!("Capacity: {}", fruits.capacity());

    // Try to add beyond capacity
    fruits.insert(String::from("Mango"), 4); // This will work but may reallocate
    println!("After adding beyond capacity:");
    println!("Fruits: {:?}", fruits);
    println!("New capacity: {}", fruits.capacity()); // Capacity will likely have increased
}

// level _3
fn level_3()
{
    println!("\nDirect HashMap Initialization Examples:");

    // Sabse basic example - direct initialization
    let simple_map = HashMap::from([
        (String::from("Samosa"), 15),
        (String::from("Kachori"), 12),
    ]);
    println!("Basic menu prices: {:?}", simple_map);

    // Thoda complex example with different types
    let student_details = HashMap::from([
        ("Roll_No", "42"),
        ("Name", "Kush"),
        ("Branch", "CSE"), 
    ]);
    println!("\nStudent ka data: {:?}", student_details);

    // Advanced example with nested data
    let complex_map = HashMap::from([
        ("config", HashMap::from([
            ("port", "8080"),
            ("host", "localhost")
        ])),
        ("metadata", HashMap::from([
            ("version", "1.0"),
            ("author", "Kush")
        ]))
    ]);
    println!("\nNested configuration: {:?}", complex_map);

    
}