enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// let home = IpAddr::V4(127, 0, 0, 1);
//
// let loopback = IpAddr::V6(String::from("::1"));

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _some_number = Some(5);
    let _some_string = Some("a_string");
    let _absent_number: Option<i32> = None;
}
