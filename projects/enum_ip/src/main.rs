// Enums are useful when you need to express possibilities in the type itself,
// where the value can be one of either types.
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));

    route(home);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:#?}", ip_kind);
}
