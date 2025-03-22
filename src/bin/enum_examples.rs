use std::vec;



// enum delcaration
enum TrafficLights {
    Red,
    Yellow,
    Green,
}
enum UserStatus {
    Active(String),
    Inactive(u64),
    Banned{
        reason:String,
        days:u32
    }
}

enum Message {
    Quit,                           // No data
    Move { x: i32, y: i32 },       // Anonymous struct
    Write(String),                  // String data
    ChangeColor(i32, i32, i32),    // RGB values
}
fn main() {
    
    // basic usecase of enum

    let red = TrafficLights::Red;
    let green= TrafficLights::Green;
    let yellow = TrafficLights::Yellow;

    println!("this is basic useCase`---------------------`");
    enum_example(red);
    enum_example(green);
    enum_example(yellow);

    let user_1 = UserStatus::Active(String::from("Kush")) ;
    let user_2 = UserStatus::Inactive(13);
    let user_3 = UserStatus::Banned { reason: (String::from("keep on spamming every one")), days: (56) };

    println!("`----------------------------------------------------------------`");
    enum_example_2(user_1);
    enum_example_2(user_2);
    enum_example_2(user_3);

    println!("`----------------------------------`");
    enum_example_3();
    println!("`----------------------------------`");

    let bucket= optional_enums(true);
    match bucket {
        Some(n) => println!("{}",n),
        None => println!("not found")
    }

}

fn enum_example(picked_color:TrafficLights) {
    let _colorselection = TrafficLights::Green;

    match picked_color {
        TrafficLights::Green => println!("this is green bro"),
        TrafficLights::Red => println!("this is Red bro "),
        TrafficLights::Yellow => println!("this is yellow bro"),
    }
}


fn enum_example_2(picked_status:UserStatus)
{
   
    match picked_status {
        UserStatus::Active(name) => println!("{} user is online",name),
        UserStatus::Inactive(dayss) => println!(" usr is offline since {} days", dayss),
        UserStatus::Banned { reason  , days } => {
            println!("user gets banned : {} for  days : {}",reason, days)
        }

    }
}

fn enum_example_3()
{
    let text_msg= vec![
        Message::Quit,
        Message::Move { x: (45), y: (65) },
        Message::Write(String::from("here i goesss weeee!")),
        Message::ChangeColor(255,0,0),
    ];
   

    for msges in text_msg {
        match msges {
            Message::Quit => println!("Game band karo"),
            Message::Move { x, y } => println!("Position change karo: x={}, y={}", x, y),
            Message::Write(text) => println!("Message likho: {}", text),
            Message::ChangeColor(r, g, b) => println!("Rang badlo: RGB({},{},{})", r, g, b),
        }
    }
}



// optional enums 

fn optional_enums(istrue :bool) -> Option<String>
{
if istrue {
    Some(String::from("Kush"))
}
else {
    None
}
}
