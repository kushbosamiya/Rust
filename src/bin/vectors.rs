fn main()
{
    basics();
    level_1();
    level_2();
    multiple_elements_example();
}

fn basics()
{
    // empty vectors
    let mut number_arr = Vec::new();

    number_arr.push("Kush");
    number_arr.push("Bosamiya");
    // to acces the whole vectors and index of the vectors
    println!("{:?} is my full name and this is the 1st name : {} // this is usage of empty and macro vectors",&number_arr,&number_arr[1]);


    // macro vector
    let macro_arr =vec![2,45,65, 78];
    println!("this is macro access :{:?}",&macro_arr);

    // preallocated vectors
    let arr_capa: Vec<i32> = Vec::with_capacity(3);

    println!("capacity : {} // example of the capacity keyword",arr_capa.capacity());
}

fn level_1()
{
    let mut number_arr = Vec::new();

    number_arr.push("Kush");
    number_arr.push("Bosamiya");

    // so the get function always returns the option 

    match number_arr.get(0)
    {
        Some(value)=> println!("this is called usin get : {}",value),
        None => println!("OUT OF BOUND"),
    }

    println!("capacity : {} and lenght : {}",&number_arr.capacity(),&number_arr.len());

    // vector slicing 

    let num_arr2= vec![30,69,58,45,77];

    let slice = &num_arr2[1..3];
    println!("{:?}",slice);
}
fn level_2() {
    // Example of buffer resize using resize()
    let mut buffer = vec![1, 2, 3];
    println!("Original buffer: {:?}", buffer);

    // Resize to length 5, filling new elements with 0
    buffer.resize(5, 0);
    println!("After resize to 5: {:?}", buffer);

    // Resize to smaller length 2
    buffer.resize(2, 0);  // Fixed: Can only use single fill value
    println!("After resize to 2: {:?}", buffer);

    // Resize with a different fill value
    buffer.resize(4, 42);
    println!("After resize to 4 with value 42: {:?}", buffer);
}

// New function to demonstrate multiple elements addition
fn multiple_elements_example() {
    println!("\nMultiple Elements Addition Examples:");
    
    // Method 1: Using extend
    let mut vec1 = vec![1, 2];
    vec1.extend(vec![56, 54, 98]);
    println!("After extend: {:?}", vec1);

    // Method 2: Using extend_from_slice
    let mut vec2 = vec![1, 2];
    vec2.extend_from_slice(&[56, 54, 98]);
    println!("After extend_from_slice: {:?}", vec2);

    // Method 3: Using append
    let mut vec3 = vec![1, 2];
    let mut temp = vec![56, 54, 98];
    vec3.append(&mut temp);
    println!("After append: {:?}", vec3);

    // Method 4: Using vec! macro with multiple elements
    let vec4 = vec![1, 2, 56, 54, 98];
    println!("Using vec! macro: {:?}", vec4);
}




