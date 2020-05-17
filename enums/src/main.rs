use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKindValues {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddrTest {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Using STRUCTS to organize the various IP addresses
    let home = IpAddrTest {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrTest {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home {:#?}, loopback {:#?}", home, loopback);

    //Putting value DIRECTLY on the Enum
    let home2 = IpAddrKindValues::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrKindValues::V6(String::from("::1"));

    println!("home2 {:#?}, loopback2 {:#?}", home2, loopback2);

    //Using the standard library IP address storage system
    let loc_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!("127.0.0.1".parse(), Ok(loc_v4));

    // enum with different types in its variants
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    };

    //struct versions
    struct QuitMessage; //unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    };
    //etc etc
    // the difference here is that each struct has its own type, whereas all the enums have a shared type.

    // adding an implementation onto an enum
    impl Message {
        fn call(&self) {
            println!("more would be in the body");
        }
    };

    // calling the method
    let message_call = Message::Write(String::from("this stuff")).call();

    //Option Enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    //the following will NOT work
    // let sum = 5 + some_number;
    let new_number = some_number.unwrap_or(9);

    //this will work after unwrapping the SOME
    let sum = 5 + new_number;

    println!("10 === {}", sum);
}
