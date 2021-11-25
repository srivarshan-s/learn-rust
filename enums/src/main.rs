// Defining an Enum

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Enum values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddrEnum {
        V4(String),
        V6(String),
    }
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    enum IpAddrU8 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrU8::V4(127, 0, 0, 1);
    let loopback = IpAddrU8::V6(String::from("::1"));

    struct IpV4Addr {}
    struct IpV6Addr {}
    enum IpAddrStruct {
        V4(IpV4Addr),
        V6(IpV6Addr),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            println!("Function call() called");
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum
    // Already declared and brought into scope
    // enum Option<T> {
    //    None,
    //    Some(T),
    // }
    let some_number = Some(5);
    let som_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Won't compile as we are trying to add i8 to Option<i8>
    // let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {}
