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
    println!("## Chapter 6");

    /* ######################################################################### */
    /* 6.1                                                                       */
    /* ######################################################################### */
    println!("#### 6.1");


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


    /* ######################################################################### */
    /* 6.2                                                                       */
    /* ######################################################################### */
    println!("\n#### 6.2");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            },
        }
    }

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    

    println!("penny = {}", value_in_cents(&penny));
    println!("nickel = {}", value_in_cents(&nickel));
    println!("dime = {}", value_in_cents(&dime));
    println!("alabama_quarter = {}", value_in_cents(&alabama_quarter));
    println!("alaska_quarter = {}", value_in_cents(&alaska_quarter));

    let dice_roll = 3;
    match dice_roll {
        3 => println!("a fancy hat!"),
        7 => println!("removing hat :("),
        _ => () // nothing
    }


    /* ######################################################################### */
    /* 6.3                                                                       */
    /* ######################################################################### */
    println!("\n#### 6.3");


    let mut config_max = Some(3u8);

    match config_max {
        Some(max) => println!("max configured = {max}"),
        _ => () // boilerplate if not needed.
    }

    /*
        instead, we can use if let.
    */
    config_max = None;
    if let Some(max) = config_max {
        println!("max configured = {max}");
    }
    else {
        println!("max not configured");
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
            }
        }
    }

    fn describe_state_quarter(coin: &Coin) -> Option<String> {
        let state = if let Coin::Quarter(state) = coin {
            state
        } else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    println!("{}", describe_state_quarter(&alabama_quarter).unwrap());
    println!("{}", describe_state_quarter(&alaska_quarter).unwrap());
}
