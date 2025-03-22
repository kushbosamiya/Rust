fn main()
{
example();
exmaple_2();
example_3();
struct_implementation();
}

// lets learn structs 


fn example()
{
    //  STEP:1 -> here first we design the blueprint of the object 
    struct User {
    name:String,
    email :String,
    // active: bool
    }
    // STEP :2  -> nothing but the putting the values inside the blueprint

    // commonly used structs 
    let student = User {
        name: String::from("my name is Kush"),
        email : String::from("kush@gmail.com"),
        // active:true
    };
    println!("my name is {} and this is my mail:{}",student.name , student.email);
}

// tuple structs

fn exmaple_2()
{
    // define data type of props
    struct BoxSize(u8,u8);

    let actual_size =BoxSize(8,6);
    println!("total size of box : {} & this is tuple struct",actual_size.0* actual_size.1);

    // -> its only their for to follow the orders
}

// -> there is also an unlike structs which only we use as a marker  not being stored in memory and nothing gets stored inside the structs

fn example_3()
{
    // unit like structs
    struct AlwaysTrue; // No fields!
    
    let flag = AlwaysTrue;
    
    // Example usage:
    fn verify_truth(_: AlwaysTrue) {
        println!("This can only be called with AlwaysTrue type!");
    }
    
    verify_truth(flag);  // Works
    // verify_truth(123);  // Would not compile - wrong type
}



// struct implementation 
fn struct_implementation()
{
    struct Grade {
        age:u32,
        class:u32,
        enrollment:u32,
    }

    impl Grade {
        fn data(&self) -> u32
        {
            self.age + self.class + self.enrollment
        }
    }
    let collective_class = Grade {age:5, class:1, enrollment:30};
    println!("Age: {}, Class: {}, Enrollment: {}, Total: {}", 
        collective_class.age, 
        collective_class.class, 
        collective_class.enrollment,
        collective_class.data()
    );
}