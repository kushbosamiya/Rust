


fn main() {
    // println!("this is my fist program in rust!");
    
  
    
    // for numbers
    let result = sum(5,10);
    println!("{}",result);

    // booleans 
    println!("This is the useCase of Boolean : {}",is_odd(3));

    // string
    let content: &str ="This is useCase for string literals" ;
    println!("{}",content);

    let mut dynamic = String::from("this is growable string");
    dynamic.push_str("and this is important");
    println!("{}",dynamic);

    // concept of mutablity : CASE_1 :by default all the rust variable are the immuatble in nature 
    let bucket :i32 =7;
    println!("this is by default mutable in nature :{}",bucket);
    // -> hence we cant change the value of existing variable once its assigned , if it happens it would throws the error : `Error: cannot assign to immutable variable`

    // CASE_2 : want to change the value of variable for that plse use `mut` as keyword
    let mut thiswillchange = 10; // Mutable variable
    println!("Initial data: {}", thiswillchange);
    
    thiswillchange = 20; // ✅ Works fine
    println!("Updated data: {}", thiswillchange);

    // vectors literals
    let kaam_chalau =vec![4,5,6,7,54];
    println!("these are the vector literals :{:?}",kaam_chalau);

    // empty vectors 
    let mut tasks: Vec<&str> = Vec::new();
    tasks.push("this is updating the vectors ");
    tasks.push("this is final updated task");

    println!("this is dynamic vector :{:?}",tasks)

    // we can aacces the elements of the vectors same as accesing the elems of Array

}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn is_odd(a:u32) -> bool
{
    return a%2==0;
}


