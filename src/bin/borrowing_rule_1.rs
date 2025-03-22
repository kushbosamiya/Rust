



fn main()
{
multiple_borrowers();
}

// should have one owner but other than this it could have multiple borrowers 

fn multiple_borrowers()
{
    let string_storage = String::from("this could be accesed by multiple people");

    let another_storage_1 = &string_storage; // this can only have read permission

    let another_storage_2 = &string_storage;

    // let mega_bucket = String::from("1:") + another_storage_1 + " " + "2:" + another_storage_2 + " ";
    // -> in rust we can only does concatenation if starts from the string::from


    let mega_bucket = format!("{} {}",another_storage_1 , another_storage_2) ;

    println!("{}",mega_bucket);

}