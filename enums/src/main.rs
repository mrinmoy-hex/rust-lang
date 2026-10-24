#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(localhost);
    route(loopback);
}

fn route(ip: IpAddrKind) {
    println!("Routing request to {:?}", ip);
}

