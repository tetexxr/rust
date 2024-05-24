fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

fn first_implementation() {
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn second_implementation() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.2"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn third_implementation() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn fourth_implementation() {
    // This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

// enum with four variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }

    // let m = Message::Write(String::from("hello"));
    // m.call();
}

// the same in structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent
// enum Option<T> {
//     None,
//     Some(T),
// }

fn example_for_option() {
    let some_number = Some(5);
    let some_char = Some('e');
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
}
