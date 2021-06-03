#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("Call of {:?}", self);
        use IpAddr::*;
        match self {
            V4(_, _, _, _) => println!("v4: {:?}", self),
            V6(s) => println!("v6: {:?}", s),
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChageColor(i32, i32, i32),
    Connect(IpAddr),
}

fn main() {
    let v4 = IpAddr::V4(192, 168, 0, 1);
    let v6 = IpAddr::V6(String::from("::1"));
    println!("The value of v4 is {:?}", v4);
    println!("The value of v6 is {:?}", v6);

    v4.call();
    v6.call();

    // Message
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 32, y: 32 };
    let write = Message::Write(String::from("hello"));
    let change_color = Message::ChageColor(3, 3, 3);
    let connect = Message::Connect(IpAddr::V6(String::from("::1")));

    println!("The message of quit is {:?}", quit);
    println!("The message of move_msg is {:?}", move_msg);
    println!("The message of write is {:?}", write);
    println!("The message of change_color is {:?}", change_color);
    println!("The message of connect is {:?}", connect);

    // option
    let some_u8 = Some(6);
    match some_u8 {
        Some(v) => println!("Option::Some: {}", v),
        None => (),
    }
    match some_u8 {
        Some(v) => println!("Option::Some: {}", v),
        _ => (),
    }
    if let Some(d) = some_u8 {
        println!("The value of d is {}", d);
    }
}
