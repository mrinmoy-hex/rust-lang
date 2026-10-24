#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    address: String,
    kind: IpAddrKind,
}

impl IpAddress {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IpAddrKind::V4,
        }
    }
}

fn main() {
    let google_address = IpAddress {
        address: String::from("192.168.2.3"),
        kind: IpAddrKind::V4,
    };
    let google_addr = IpAddress::new("192.168.2.3");
    let loopback = IpAddress::new("127.0.0.1");

    route(google_addr);
}

fn route(ip: IpAddress) {
    println!(
        "Routing request to IP: {} of kind {:?}",
        ip.address, ip.kind
    );
}