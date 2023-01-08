fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Using Option enum from the standard library
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // This wont work because Option<T> and T are different types!
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method body
    }
}

fn route(ip_kind: IpAddrKind) {}
