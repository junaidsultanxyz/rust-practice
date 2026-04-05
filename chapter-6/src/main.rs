#[derive(Debug)]
enum IpKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddressStruct {
    kind: IpKind,
    address: String
}

/*
    instead of making a struct to handle the data, enums can do the same thing now.
    This means, when creating IpAddressEnum now, we can give the string value while
    mentioning the type.
*/
#[derive(Debug)]
enum IpAddressEnum {
    /*
        we can put any type of data inside an enum varient
    */
    V4(u8, u8, u8, u8),
    // ^^^^^^^^^^^^^^ varient
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x:i32, y: i32}, // names fields like struct
    Write(String),
    ChangeColor (i32, i32, i32)
}

impl Message {

    fn call (&self) {
        println!("message called: {self:?}");
    }
}

fn main() { 
    let pc_ip_local = IpAddressStruct {
        kind: IpKind::V4,
        address: String::from("192.168.100.10")
    };

    let server_ip_local = IpAddressStruct {
        kind: IpKind::V4,
        address: String::from("192.168.100.1")
    };

    let loopback = IpAddressStruct {
        kind: IpKind::V6,
        address: String::from("::1")
    };

    println!("pc ip = {pc_ip_local:?}");
    println!("server ip = {server_ip_local:?}");
    println!("loopback ip = {loopback:?}");


    let mobile_ip_local = IpAddressEnum::V4(192, 168, 100, 1);
    let loopback2 = IpAddressEnum::V6(String::from("::1"));
    
    println!("mobile ip: {mobile_ip_local:?}");
    println!("loopback2: {loopback2:?}");


    let message1 = Message::Write(String::from("this is a write message"));
    message1.call();

    let value: u8 = 200;
    let some_value: Option<u8> = Some(10);
    let _some_other_value = Some("yes");
    let _other_number: Option<i32> = None; // writing None will require to explicitly write Option with type

    let sum = value + match some_value {
        Some(u8) => { u8 },
        None => { 
            println!("ERROR while adding. One value is null");
            0
        }
    };

    let some_value_converted: u8 = some_value.expect("some_value is None");
    let sum2 = value + some_value_converted;
    /*
        simply trying to add won't work.
    */
    
    println!("sum of {value} + {some_value:?} = {sum}");
    println!("sum of {value} + {some_value_converted} (converted) = {sum2}");
}
