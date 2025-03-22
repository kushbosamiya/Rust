fn main()
{
mutabel_ref();
}

// owner should be their but other than this , if some variable  accepts the mutable references then , no one other than itself can view that variable 

fn mutabel_ref()
{
    let mut storage_bucket = String::from("mutable reference of rihana");

    let another_bucket = &mut storage_bucket;
  
    
    println!("{}", another_bucket);

// Mutable (&mut) and immutable (&) references CANNOT exist at the same time. , but even we want to do that , we want to do that in seperate block scope 

}